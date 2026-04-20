use std::ops::{Add, Div, Sub};
use crate::math::traits::mul::MulId;

/// Marker trait for integer types.
pub trait Integer {}

/// Marker trait for non-integer numeric types.
pub trait NonInteger {}

pub trait IntDiv: Integer {
    fn int_div(self, rhs: Self) -> Self;
}

pub trait Inc: Integer {
    fn inc(self) -> Self;
}

pub trait Dec: Integer {
    fn dec(self) -> Self;
}

impl<T> IntDiv for T
where
    T: Integer + Div<Output = T>,
{
    fn int_div(self, rhs: Self) -> Self {
        self / rhs
    }
}

impl<T> Inc for T
where
    T: Integer + MulId + Add<Output = T>,
{
    fn inc(self) -> Self {
        self + T::ONE
    }
}

impl<T> Dec for T
where
    T: Integer + MulId + Sub<Output = T>,
{
    fn dec(self) -> Self {
        self - T::ONE
    }
}

macro_rules! impl_integer_markers {
    ($($t:ty),* $(,)?) => {
        $(
            impl Integer for $t {}
        )*
    };
}

macro_rules! impl_non_integer_markers {
    ($($t:ty),* $(,)?) => {
        $(
            impl NonInteger for $t {}
        )*
    };
}

impl_integer_markers!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_non_integer_markers!(f32, f64);
