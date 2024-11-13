mod get_dep;
mod subscription;

// macros
pub use init_system_macro::InitSystem;

pub use get_dep::*;
pub use subscription::*;

pub trait InitTask {
    type Deps;
    type InitDeps;
    type Res;
    fn init(init: Self, deps: Self::Deps, init_deps: Self::InitDeps) -> Self::Res;
}
