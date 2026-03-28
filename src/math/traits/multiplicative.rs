pub trait MultId {
    const ONE:Self;
}
///Implementation of MultID for basic types
macro_rules! impl_mult_id_int {
    ($($t:ty),*$(,)?) => {
        $(
            impl MultId for $t {
                const ONE: $t = 1;
            }
        )*
    };
}
macro_rules! impl_mult_id_float {
    ($($t:ty),*$(,)?) => {
        $(
            impl MultId for $t {
                const ONE: $t = 1.0;
            }
        )*
    };
}
impl_mult_id_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, u128, i128);
impl_mult_id_float!(f32, f64);

pub trait Recip{
    fn recip(self) -> Self;
}
///Implementation of Recip for basic types
macro_rules! impl_recip_float{
    ($($t:ty),*$(,)?) => (
        $(
            impl Recip for $t{
                fn recip(self) -> Self {
                    Self::recip(self)
                }
            }
        )*
    )
}
impl_recip_float!(f32, f64);


///Trait for checking if a number is NaN
pub trait IsNaN{
    fn is_nan(&self) -> bool;
    fn not_nan(&self) -> bool;
}
///Auto-Implementation of CheckNaN
impl<T:PartialEq> IsNaN for T{
    fn is_nan(&self) -> bool {
        self!=self
    }
    fn not_nan(&self) -> bool {
        self==self
    }
}

///Marker trait for types that can be NaN
pub trait NaN{
    const NAN:Self;
}
impl NaN for f32{ const NAN: Self = f32::NAN; }
impl NaN for f64{ const NAN: Self = f64::NAN; }

