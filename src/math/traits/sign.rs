use std::cmp::Ordering;
use std::ops::Neg;
use crate::math::IsAddId;
use crate::math::traits::AddId;

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
impl From<Sign> for i8 {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Negative => -1,
            Sign::Zero => 0,
            Sign::Positive => 1,
        }
    }
}
impl From<i8> for Sign{
    fn from(value: i8) -> Self {
        if value < 0 {
            Sign::Negative
        }else if value == 0 {
            Sign::Zero
        }else{
            Sign::Positive
        }
    }
}

pub trait HasPartialSign: PartialOrd+IsAddId {
    fn partial_sign(&self) -> Option<Sign>;
    fn is_positive(&self) -> bool {
        self.partial_sign() == Some(Sign::Positive)
    }

    fn is_negative(&self) -> bool {
        self.partial_sign() == Some(Sign::Negative)
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
}

pub trait HasSign: HasPartialSign + Ord {
    fn sign(&self) -> Sign;
}
pub trait Signum: PartialOrd{
     fn signum(&self) ->Self;
}
pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

pub trait Signed: Neg<Output = Self> {}
pub trait Unsigned {}

#[cfg(feature = "specialization")]
impl<T> HasPartialSign for T
where
    T: PartialOrd + AddId+IsAddId,
{
    fn partial_sign(&self) -> Option<Sign> {
        match self.partial_cmp(&T::ZERO) {
            Some(x) => Some(x.into()),
            None => None,
        }
    }
}

#[cfg(feature = "specialization")]
impl<T> HasSign for T
where
    T: HasPartialSign + Ord+AddId,
{
    fn sign(&self) -> Sign {
        self.cmp(&T::ZERO).into()
    }
}

macro_rules! impl_has_partial_sign_primitive {
    ($($t:ty),* $(,)?) => {
        $(
            #[cfg(not(feature = "specialization"))]
            impl HasPartialSign for $t {
                fn partial_sign(&self) -> Option<Sign> {
                    match self.partial_cmp(&Self::ZERO) {
                        Some(x) => Some(x.into()),
                        None => None,
                    }
                }
            }
        )*
    };
}

macro_rules! impl_has_sign_primitive {
    ($($t:ty),* $(,)?) => {
        $(
            #[cfg(not(feature = "specialization"))]
            impl HasSign for $t {
                fn sign(&self) -> Sign {
                    self.cmp(&Self::ZERO).into()
                }
            }
        )*
    };
}

impl_has_partial_sign_primitive!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
impl_has_sign_primitive!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

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
