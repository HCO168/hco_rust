#[cfg(feature = "specialization")]
use std::ops::{Add, Div, Sub};
#[cfg(feature = "specialization")]
use std::ops::Rem;
#[cfg(feature = "specialization")]
use crate::math::{Abs, AddId, HasPartialSign};
use crate::math::{gcd_euclid_iterative, gcd_stein};
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

#[cfg(feature = "specialization")]
impl<T> IntDiv for T
where
    T: Integer + Div<Output = T>,
{
    fn int_div(self, rhs: Self) -> Self {
        self / rhs
    }
}

#[cfg(feature = "specialization")]
impl<T> Inc for T
where
    T: Integer + MulId + Add<Output = T>,
{
    fn inc(self) -> Self {
        self + T::ONE
    }
}

#[cfg(feature = "specialization")]
impl<T> Dec for T
where
    T: Integer + MulId + Sub<Output = T>,
{
    fn dec(self) -> Self {
        self - T::ONE
    }
}

macro_rules! impl_integer_ops {
    ($($t:ty),* $(,)?) => {
        $(
            #[cfg(not(feature = "specialization"))]
            impl IntDiv for $t {
                fn int_div(self, rhs: Self) -> Self {
                    self / rhs
                }
            }

            #[cfg(not(feature = "specialization"))]
            impl Inc for $t {
                fn inc(self) -> Self {
                    self + Self::ONE
                }
            }

            #[cfg(not(feature = "specialization"))]
            impl Dec for $t {
                fn dec(self) -> Self {
                    self - Self::ONE
                }
            }
        )*
    };
}

impl_integer_ops!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

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

pub trait TrailingZeros{
    fn trailing_zeros(self) -> usize;
}
macro_rules! impl_trailing_zeros {
    ($($t:ty),* $(,)?) => {
        $(
            impl TrailingZeros for $t {
                fn trailing_zeros(self) -> usize {
                    Self::trailing_zeros(self) as usize
                }
            }
        )*
    }
}
impl_trailing_zeros!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
pub trait LeadingZeros{
    fn leading_zeros(self) -> usize;
}
macro_rules! impl_leading_zeros {
    ($($t:ty),* $(,)?) => {
        $(
            impl LeadingZeros for $t {
                fn leading_zeros(self) -> usize {
                    Self::leading_zeros(self) as usize
                }
            }
        )*
    }
}
impl_leading_zeros!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
pub trait GCD{
    fn gcd(self, other: Self) -> Self;
}
macro_rules! impl_gcd_by_fn {
    ($f:path => $($t:ty),* $(,)?) => {
        $(
            impl GCD for $t {
                fn gcd(self, other: Self) -> Self {
                    $f(self, other)
                }
            }
        )*
    };
}
impl_gcd_by_fn!(gcd_stein=>i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_gcd_by_fn!(gcd_euclid_iterative=>f32, f64);
#[cfg(feature = "specialization")]
impl<T:AddId+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone + HasPartialSign> GCD for T{
    default fn gcd(self, other: Self) -> Self {
        gcd_euclid_iterative(self,other)
    }
}
