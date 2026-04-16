pub trait MaybeNaN{}

pub trait NeverNaN{}

pub trait IsNaN {
    fn is_nan(&self) -> bool;

    fn not_nan(&self) -> bool {
        !self.is_nan()
    }
}

impl MaybeNaN for f32 {}
impl IsNaN for f32 {
    fn is_nan(&self) -> bool {
        Self::is_nan(*self)
    }
}
impl MaybeNaN for f64 {}
impl IsNaN for f64 {
    fn is_nan(&self) -> bool {
        Self::is_nan(*self)
    }
}
macro_rules! impl_never_nan {
    ($($t:ty),* $(,)?) => {
        $(
            impl NeverNaN for $t {}
        )*
    };
}
impl_never_nan!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);