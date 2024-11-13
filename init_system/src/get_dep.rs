pub trait GetDep<T> {
    fn get(&mut self) -> T;
}

impl<C> GetDep<()> for C {
    #[inline(always)]
    fn get(&mut self) {}
}

impl<C, T1> GetDep<(T1,)> for C
where
    C: GetDep<T1>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1,) {
        (self.get(),)
    }
}

impl<C, T1, T2> GetDep<(T1, T2)> for C
where
    C: GetDep<T1> + GetDep<T2>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2) {
        (self.get(), self.get())
    }
}

impl<C, T1, T2, T3> GetDep<(T1, T2, T3)> for C
where
    C: GetDep<T1> + GetDep<T2> + GetDep<T3>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3) {
        (self.get(), self.get(), self.get())
    }
}

impl<C, T1, T2, T3, T4> GetDep<(T1, T2, T3, T4)> for C
where
    C: GetDep<T1> + GetDep<T2> + GetDep<T3> + GetDep<T4>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4) {
        (self.get(), self.get(), self.get(), self.get())
    }
}

impl<C, T1, T2, T3, T4, T5> GetDep<(T1, T2, T3, T4, T5)> for C
where
    C: GetDep<T1> + GetDep<T2> + GetDep<T3> + GetDep<T4> + GetDep<T5>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5) {
        (self.get(), self.get(), self.get(), self.get(), self.get())
    }
}

impl<C, T1, T2, T3, T4, T5, T6> GetDep<(T1, T2, T3, T4, T5, T6)> for C
where
    C: GetDep<T1> + GetDep<T2> + GetDep<T3> + GetDep<T4> + GetDep<T5> + GetDep<T6>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5, T6) {
        (self.get(), self.get(), self.get(), self.get(), self.get(), self.get())
    }
}

impl<C, T1, T2, T3, T4, T5, T6, T7> GetDep<(T1, T2, T3, T4, T5, T6, T7)> for C
where
    C: GetDep<T1> + GetDep<T2> + GetDep<T3> + GetDep<T4> + GetDep<T5> + GetDep<T6> + GetDep<T7>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5, T6, T7) {
        (self.get(), self.get(), self.get(), self.get(), self.get(), self.get(), self.get())
    }
}

impl<C, T1, T2, T3, T4, T5, T6, T7, T8> GetDep<(T1, T2, T3, T4, T5, T6, T7, T8)> for C
where
    C: GetDep<T1> + GetDep<T2> + GetDep<T3> + GetDep<T4> + GetDep<T5> + GetDep<T6> + GetDep<T7> + GetDep<T8>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5, T6, T7, T8) {
        (self.get(), self.get(), self.get(), self.get(), self.get(), self.get(), self.get(), self.get())
    }
}


impl<C, T1, T2, T3, T4, T5, T6, T7, T8, T9> GetDep<(T1, T2, T3, T4, T5, T6, T7, T8, T9)> for C
where
    C: GetDep<T1> + GetDep<T2> + GetDep<T3> + GetDep<T4> + GetDep<T5> + GetDep<T6> + GetDep<T7> + GetDep<T8> + GetDep<T9>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5, T6, T7, T8, T9) {
        (
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
        )
    }
}

impl<C, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> GetDep<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> for C
where
    C: GetDep<T1> + GetDep<T2> + GetDep<T3> + GetDep<T4> + GetDep<T5> + GetDep<T6> + GetDep<T7> + GetDep<T8> + GetDep<T9> + GetDep<T10>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
        (
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
        )
    }
}

impl<C, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> GetDep<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for C
where
    C: GetDep<T1>
    + GetDep<T2>
    + GetDep<T3>
    + GetDep<T4>
    + GetDep<T5>
    + GetDep<T6>
    + GetDep<T7>
    + GetDep<T8>
    + GetDep<T9>
    + GetDep<T10>
    + GetDep<T11>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
        (
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
        )
    }
}

impl<C, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> GetDep<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> for C
where
    C: GetDep<T1>
    + GetDep<T2>
    + GetDep<T3>
    + GetDep<T4>
    + GetDep<T5>
    + GetDep<T6>
    + GetDep<T7>
    + GetDep<T8>
    + GetDep<T9>
    + GetDep<T10>
    + GetDep<T11>
    + GetDep<T12>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
        (
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
        )
    }
}

impl<C, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> GetDep<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> for C
where
    C: GetDep<T1>
    + GetDep<T2>
    + GetDep<T3>
    + GetDep<T4>
    + GetDep<T5>
    + GetDep<T6>
    + GetDep<T7>
    + GetDep<T8>
    + GetDep<T9>
    + GetDep<T10>
    + GetDep<T11>
    + GetDep<T12>
    + GetDep<T13>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
        (
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
        )
    }
}

impl<C, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> GetDep<(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14,
)> for C
where
    C: GetDep<T1>
    + GetDep<T2>
    + GetDep<T3>
    + GetDep<T4>
    + GetDep<T5>
    + GetDep<T6>
    + GetDep<T7>
    + GetDep<T8>
    + GetDep<T9>
    + GetDep<T10>
    + GetDep<T11>
    + GetDep<T12>
    + GetDep<T13>
    + GetDep<T14>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
        (
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
        )
    }
}

// Продолжение до T30

impl<C, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> GetDep<(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15,
)> for C
where
    C: GetDep<T1>
    + GetDep<T2>
    + GetDep<T3>
    + GetDep<T4>
    + GetDep<T5>
    + GetDep<T6>
    + GetDep<T7>
    + GetDep<T8>
    + GetDep<T9>
    + GetDep<T10>
    + GetDep<T11>
    + GetDep<T12>
    + GetDep<T13>
    + GetDep<T14>
    + GetDep<T15>,
{
    #[inline(always)]
    fn get(&mut self) -> (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
        (
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
        )
    }
}

// Аналогичные имплементации до T30
// Закончим с T30
impl<
    C, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29, T30,
> GetDep<(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30,
)> for C
where
    C: GetDep<T1>
    + GetDep<T2>
    + GetDep<T3>
    + GetDep<T4>
    + GetDep<T5>
    + GetDep<T6>
    + GetDep<T7>
    + GetDep<T8>
    + GetDep<T9>
    + GetDep<T10>
    + GetDep<T11>
    + GetDep<T12>
    + GetDep<T13>
    + GetDep<T14>
    + GetDep<T15>
    + GetDep<T16>
    + GetDep<T17>
    + GetDep<T18>
    + GetDep<T19>
    + GetDep<T20>
    + GetDep<T21>
    + GetDep<T22>
    + GetDep<T23>
    + GetDep<T24>
    + GetDep<T25>
    + GetDep<T26>
    + GetDep<T27>
    + GetDep<T28>
    + GetDep<T29>
    + GetDep<T30>,
{
    #[inline(always)]
    fn get(&mut self) -> (
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23, T24, T25, T26, T27, T28, T29, T30,
    ) {
        (
            self.get(), self.get(), self.get(), self.get(), self.get(), self.get(), self.get(),
            self.get(), self.get(), self.get(), self.get(), self.get(), self.get(), self.get(),
            self.get(), self.get(), self.get(), self.get(), self.get(), self.get(), self.get(),
            self.get(), self.get(), self.get(), self.get(), self.get(), self.get(), self.get(),
            self.get(), self.get(),
        )
    }
}
