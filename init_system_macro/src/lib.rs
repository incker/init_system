extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};
use std::{fmt, mem};
use syn::parse::Parser;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Meta, Token, Type};

#[proc_macro_derive(InitSystem, attributes(skip, returns, depends))]
pub fn derive_init_system(input: TokenStream) -> TokenStream {
    // Парсим входной токен как структуру
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let ParsedFields {
        mut dep_count,
        fields,
    } = ParsedFields::new(input.data).unwrap();

    // Checks if something has reply but returns nothing (no tag #[returns])
    let mut returns_checks = Vec::new();
    // Results to fields that have #[returns]
    let mut return_fields = Vec::new();
    // Types to fields that have #[returns]
    let mut return_types = Vec::new();

    // Collecting dependencies for `InitTask`
    let mut field_gets = Vec::with_capacity(fields.len());
    // Init `InitTask` with field_gets
    let mut field_inits = Vec::with_capacity(fields.len());

    for field in fields {
        let InitSystemField {
            id,
            field_name,
            field_type,
            field_type_str,
            attr_skip,
            attr_returns,
            attr_depends,
            is_dependency,
        } = field;

        let init_deps = {
            let mut init_deps = Vec::new();
            for depends in attr_depends {
                let (id, should_clone) = dep_count.dec_dependency(&depends).unwrap();
                let ident = format_ident!("resp_{}", id);
                init_deps.push(if should_clone {
                    quote! { #ident.clone() }
                } else {
                    quote! { #ident }
                });
            }
            if init_deps.is_empty() {
                quote! { () }
            } else {
                quote! { (#(#init_deps),*) }
            }
        };

        let res_variable_mane = format_ident!("resp_{}", id);

        if attr_returns {
            return_fields.push(quote! {
                #res_variable_mane
            });

            if attr_skip {
                // Will return just a field
                return_types.push(quote! {
                    #field_type
                });
                field_inits.push(quote! {
                    let #res_variable_mane = self.#field_name;
                });
            } else {
                return_types.push(quote! {
                    <#field_type as init_system::InitTask>::Res
                });

                field_gets.push(quote! {
                    let #field_name = init_system::GetDep::<<#field_type as init_system::InitTask>::Deps>::get(&mut self);
                });

                // always `is_dependency` because `attr_returns`
                field_inits.push(quote! {
                    let #res_variable_mane: <#field_type as init_system::InitTask>::Res = init_system::InitTask::init(self.#field_name, #field_name, #init_deps);
                });
            }
        } else if attr_skip {
            if is_dependency {
                field_inits.push(quote! {
                        let #res_variable_mane = self.#field_name;
                    });
            }
        } else {
            if !is_dependency {
                let err_msg = format!("Looks like init `{}` replies with some data, but no one use it.", field_type_str);
                returns_checks.push(quote! {
                       const _: () = assert!(size_of::<<#field_type as init_system::InitTask>::Res>() == 0, #err_msg);
                    });
            }

            field_gets.push(quote! {
                    let #field_name = init_system::GetDep::<<#field_type as init_system::InitTask>::Deps>::get(&mut self);
                });

            field_inits.push(quote! {
                    let #res_variable_mane: <#field_type as init_system::InitTask>::Res = init_system::InitTask::init(self.#field_name, #field_name, #init_deps);
                });
        }
    }

    // Если есть поля с #[returns], создаем кортеж, иначе возвращаем ()
    let return_type = if return_types.is_empty() {
        quote! { () }
    } else {
        quote! { (#(#return_types),*) }
    };

    let return_expr = if return_fields.is_empty() {
        quote! { () }
    } else {
        quote! { (#(#return_fields),*) }
    };

    // Условное добавление атрибута #[must_use]
    let must_use_attr = if !return_types.is_empty() {
        quote! { #[must_use] }
    } else {
        quote! {}
    };

    // Генерация реализации метода `init` для структуры
    let expanded = quote! {
        impl #name {
            #must_use_attr
            pub fn init(mut self) -> #return_type {
                #(#returns_checks)*

                #(#field_gets)*

                #(#field_inits)*

                #return_expr
            }
        }
    };

    // Превращаем код в TokenStream
    TokenStream::from(expanded)
}

pub(crate) struct ParsedFields {
    pub(crate) dep_count: DependencyCount,
    pub(crate) fields: Vec<InitSystemField>,
}

impl ParsedFields {
    fn new(data: Data) -> Result<Self, String> {
        // Имя структуры

        // Проверяем, что это структура с именованными полями
        let fields = if let Data::Struct(data) = data {
            match data.fields {
                Fields::Named(fields) => fields.named,
                _ => unimplemented!("InitSystem can only be derived for structs with named fields"),
            }
        } else {
            unimplemented!("InitSystem can only be derived for structs");
        };

        let mut dep_count = DependencyCount::with_capacity(fields.len());
        let mut parsed_fields = Vec::with_capacity(fields.len());

        for (id, field) in fields.into_iter().enumerate() {
            let field = InitSystemField::new(id as u32, field)?;
            for dep in &field.attr_depends {
                dep_count.inc_dependency(dep)?
            }
            dep_count.insert(field.field_type_str.clone(), id as u32, field.attr_returns)?;
            parsed_fields.push(field);
        }

        let mut fields_that_not_dependency = dep_count.remove_by(0);

        for field in parsed_fields.iter_mut() {
            field.is_dependency = !fields_that_not_dependency.remove(&field.field_type_str);
        }

        if !fields_that_not_dependency.is_empty() {
            panic!("Something went wrong `!fields_that_not_dependency.is_empty()`");
        }
        Ok(Self {
            dep_count,
            fields: parsed_fields,
        })
    }
}

pub(crate) struct InitSystemField {
    pub(crate) id: u32,
    pub(crate) field_name: Ident,
    pub(crate) field_type: Type,
    pub(crate) field_type_str: FieldTypeStr,
    // Проверяем, есть ли атрибут #[skip] у поля
    pub(crate) attr_skip: bool,
    // Проверяем, есть ли атрибут #[returns] у поля
    pub(crate) attr_returns: bool,
    // Проверяем, есть ли атрибут #[depends] у поля
    pub(crate) attr_depends: Vec<FieldTypeStr>,
    pub(crate) is_dependency: bool,
}

impl InitSystemField {
    fn new(id: u32, field: Field) -> Result<Self, String> {
        let field_name = if let Some(ident) = field.ident {
            ident
        } else {
            return Err("InitSystem works only for structs".to_string());
        };

        let field_type_str = FieldTypeStr::new(&field.ty);

        let mut parsed_attrs = Self {
            id,
            field_name,
            field_type: field.ty,
            field_type_str,
            attr_skip: false,
            attr_returns: false,
            attr_depends: vec![],
            is_dependency: false,
        };

        for attr in field.attrs {
            match attr.meta {
                Meta::Path(path) => {
                    if !parsed_attrs.attr_skip && path.is_ident("skip") {
                        parsed_attrs.attr_skip = true;
                        continue;
                    }
                    if !parsed_attrs.attr_returns && path.is_ident("returns") {
                        parsed_attrs.attr_returns = true;
                        continue;
                    }
                }
                Meta::List(meta_list) => {
                    if !meta_list.path.is_ident("depends") {
                        unreachable!();
                    }

                    // Парсим токены как аргументы
                    let args_parser = syn::punctuated::Punctuated::<syn::Path, Token![,]>::parse_terminated;
                    if let Ok(args) = args_parser.parse2(meta_list.tokens) {
                        for arg in args {
                            // Извлекаем идентификатор из аргумента (Path)
                            if let Some(segment) = arg.segments.first() {
                                let ident = &segment.ident;
                                parsed_attrs.attr_depends.push(FieldTypeStr(ident.to_string()));
                            }
                        }
                    } else {
                        return Err("Failed to parse arguments.".to_string());
                    }
                }
                Meta::NameValue(_) => {}
            }
        }
        Ok(parsed_attrs)
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct FieldTypeStr(pub String);

impl FieldTypeStr {
    pub fn new(field_type: &Type) -> Self {
        Self(quote! { #field_type }.to_string())
    }
}

impl fmt::Debug for FieldTypeStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.0.as_str(), f)
    }
}

impl fmt::Display for FieldTypeStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.0.as_str(), f)
    }
}

#[derive(Default)]
pub(crate) struct DependencyCount {
    inner: HashMap<FieldTypeStr, (u32, usize)>,
}

impl DependencyCount {
    pub fn with_capacity(capacity: usize) -> Self {
        Self { inner: HashMap::with_capacity(capacity) }
    }

    pub fn insert(&mut self, field_type_str: FieldTypeStr, id: u32, returns: bool) -> Result<(), String> {
        if self.inner.insert(field_type_str.clone(), (id, returns.into())).is_none() {
            Ok(())
        } else {
            Err(format!("property with type `{field_type_str} already exist`"))
        }
    }

    pub fn inc_dependency(&mut self, field_type_str: &FieldTypeStr) -> Result<(), String> {
        if let Some((_id, count)) = self.inner.get_mut(field_type_str) {
            *count += 1;
            Ok(())
        } else {
            Err(format!("property with type `{field_type_str}` should be higher than usage #[depends({field_type_str})]"))
        }
    }

    pub fn dec_dependency(&mut self, field_type_str: &FieldTypeStr) -> Option<(u32, ShouldClone)> {
        let (id, count) = self.inner.get_mut(field_type_str)?;
        let id = *id;
        if *count == 1 {
            self.inner.remove(field_type_str).expect("bug");
            Some((id, false))
        } else {
            *count -= 1;
            Some((id, true))
        }
    }

    pub fn remove_by(&mut self, target_count: usize) -> HashSet<FieldTypeStr> {
        let mut res = HashSet::with_capacity(self.inner.len());
        let mut hash_map = HashMap::with_capacity(self.inner.len());
        mem::swap(&mut self.inner, &mut hash_map);
        for (field_name, (id, count)) in hash_map {
            if target_count == count {
                res.insert(field_name);
            } else {
                self.inner.insert(field_name, (id, count));
            }
        }
        res
    }
}

pub(crate) type ShouldClone = bool;
