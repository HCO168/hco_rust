use std::cmp::Ordering;
use std::ops::Neg;

use crate::math::traits::additive::AddId;

#[repr(i8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Sign {
    Negative = -1,
    Zero = 0,
    Positive = 1,
}

impl From<Ordering> for Sign {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Less => Sign::Negative,
            Ordering::Equal => Sign::Zero,
            Ordering::Greater => Sign::Positive,
        }
    }
}

impl From<Sign> for Ordering {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Negative => Ordering::Less,
            Sign::Zero => Ordering::Equal,
            Sign::Positive => Ordering::Greater,
        }
    }
}

pub trait HasPartialSign: PartialOrd {
    fn partial_sign(&self) -> Option<Sign>;

    fn is_positive(&self) -> bool {
        self.partial_sign() == Some(Sign::Positive)
    }

    fn is_negative(&self) -> bool {
        self.partial_sign() == Some(Sign::Negative)
    }

    fn is_zero(&self) -> bool {
        self.partial_sign() == Some(Sign::Zero)
    }

    fn is_positive_or_zero(&self) -> bool {
        self.is_positive() || self.is_zero()
    }

    fn is_negative_or_zero(&self) -> bool {
        self.is_negative() || self.is_zero()
    }

    fn is_positive_or_negative(&self) -> bool {
        self.is_positive() || self.is_negative()
    }

    fn not_positive(&self) -> bool {
        !self.is_positive()
    }

    fn not_negative(&self) -> bool {
        !self.is_negative()
    }

    fn not_zero(&self) -> bool {
        !self.is_zero()
    }
}

pub trait HasSign: HasPartialSign + Ord {
    fn sign(&self) -> Sign;
}

pub trait Signum: PartialOrd {
    fn signum(&self) -> Self;
}

pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

pub trait Signed: Neg<Output = Self> {}
pub trait Unsigned {}

impl<T> HasPartialSign for T
where
    T: PartialOrd + AddId,
{
    fn partial_sign(&self) -> Option<Sign> {
        match self.partial_cmp(&T::ZERO) {
            Some(Ordering::Less) => Some(Sign::Negative),
            Some(Ordering::Equal) => Some(Sign::Zero),
            Some(Ordering::Greater) => Some(Sign::Positive),
            None => None,
        }
    }
}

impl<T> HasSign for T
where
    T: HasPartialSign + Ord,
{
    fn sign(&self) -> Sign {
        self.partial_sign().unwrap_or_else(|| todo!("HasSign called on unordered value"))
    }
}

macro_rules! impl_unsigned {
    ($($t:ty),* $(,)?) => {
        $(
            impl Unsigned for $t {}
        )*
    };
}

macro_rules! impl_signed {
    ($($t:ty),* $(,)?) => {
        $(
            impl Signed for $t {}
        )*
    };
}

macro_rules! impl_signum_signed {
    ($($t:ty),* $(,)?) => {
        $(
            impl Signum for $t {
                fn signum(&self) -> Self {
                    <$t>::signum(*self)
                }
            }
        )*
    };
}

macro_rules! impl_signum_unsigned {
    ($($t:ty),* $(,)?) => {
        $(
            impl Signum for $t {
                fn signum(&self) -> Self {
                    if *self == 0 { 0 } else { 1 }
                }
            }
        )*
    };
}

macro_rules! impl_abs_signed {
    ($($t:ty),* $(,)?) => {
        $(
            impl Abs for $t {
                type Output = $t;

                fn abs(self) -> Self::Output {
                    <$t>::abs(self)
                }
            }

            impl Abs for &$t {
                type Output = $t;

                fn abs(self) -> Self::Output {
                    <$t>::abs(*self)
                }
            }
        )*
    };
}

macro_rules! impl_abs_unsigned {
    ($($t:ty),* $(,)?) => {
        $(
            impl Abs for $t {
                type Output = $t;

                fn abs(self) -> Self::Output {
                    self
                }
            }

            impl Abs for &$t {
                type Output = $t;

                fn abs(self) -> Self::Output {
                    *self
                }
            }
        )*
    };
}

impl_unsigned!(u8, u16, u32, u64, u128, usize);
impl_signed!(i8, i16, i32, i64, i128, isize, f32, f64);
impl_signum_signed!(i8, i16, i32, i64, i128, isize, f32, f64);
impl_signum_unsigned!(u8, u16, u32, u64, u128, usize);
impl_abs_signed!(i8, i16, i32, i64, i128, isize, f32, f64);
impl_abs_unsigned!(u8, u16, u32, u64, u128, usize);
