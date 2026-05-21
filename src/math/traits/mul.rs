use std::ops::Neg;

pub trait MulId {
    const ONE: Self;
}
pub trait NegMulId{
    const NEG_ONE: Self;
}

/// Implementation of MulId for basic types.
macro_rules! impl_mult_id_int {
    ($($t:ty),*$(,)?) => {
        $(
            impl MulId for $t {
                const ONE: $t = 1;
            }
        )*
    };
}
macro_rules! impl_mult_id_float {
    ($($t:ty),*$(,)?) => {
        $(
            impl MulId for $t {
                const ONE: $t = 1.0;
            }
        )*
    };
}
impl_mult_id_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, u128, i128);
impl_mult_id_float!(f32, f64);
/// Implementation of NegMulId for basic types.
macro_rules! impl_neg_mult_id_int {
    ($($t:ty),*$(,)?) => {
        $(
            impl NegMulId for $t {
                const NEG_ONE: $t = -1;
            }
        )*
    };
}
macro_rules! impl_neg_mult_id_float {
    ($($t:ty),*$(,)?) => {
        $(
            impl NegMulId for $t {
                const NEG_ONE: $t = -1.0;
            }
        )*
    };
}
impl_neg_mult_id_int!(i8, i16, i32, i64, isize, i128);
impl_neg_mult_id_float!(f32, f64);
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

