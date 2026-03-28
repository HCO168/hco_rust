use std::cmp::Ordering;
use std::ops::Neg;


///Additive Identity, make no change when added to a value
pub trait AddId {
    const ZERO:Self;
}
///Implementation of AddID for basic types
macro_rules! impl_add_id_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl AddId for $t {
                const ZERO: $t = 0;
            }
        )*
    };
}
macro_rules! impl_add_id_float {
    ($($t:ty),* $(,)?) => {
        $(
            impl AddId for $t {
                const ZERO: $t = 0.0;
            }
        )*
    };
}

impl_add_id_int!(i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128);
impl_add_id_float!(f32, f64);


///Sign of a number
#[repr(i8)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Copy,Clone,Debug)]
pub enum Sign {
    Negative=-1,
    Zero=0,
    Positive=1,
}
impl From<Ordering> for Sign {
    fn from(value: Ordering) -> Self {
        match value{
            Ordering::Less=> Sign::Negative,
            Ordering::Equal=> Sign::Zero,
            Ordering::Greater=> Sign::Positive,
        }
    } 
}
impl From<Sign> for Ordering{
    fn from(value: Sign) -> Self {
        match value{
            Sign::Negative=>Ordering::Less,
            Sign::Zero=>Ordering::Equal,
            Sign::Positive=>Ordering::Greater,
        }
    } 
}


///Trait of getting the sign of a number
pub trait HasSign: HasPartialSign {
    fn sign(&self) -> Sign;
}
pub trait HasPartialSign {
    fn partial_sign(&self) -> Option<Sign>;
    fn is_positive(&self) -> bool{self.partial_sign()==Some(Sign::Positive)}
    fn is_negative(&self) -> bool{self.partial_sign()==Some(Sign::Negative)}
    fn is_zero(&self) -> bool{self.partial_sign()==Some(Sign::Zero)}
    fn not_positive(&self) -> bool{self.partial_sign()!=Some(Sign::Positive)}
    fn not_negative(&self) -> bool{self.partial_sign()!=Some(Sign::Negative)}
    fn not_zero(&self) -> bool{self.partial_sign()!=Some(Sign::Zero)}
}
///Auto-implementation for Get Sign & Partial Sign
impl<T> HasSign for T where T:AddId+Ord{
    fn sign(&self) -> Sign {
        if(self>&T::ZERO){
            Sign::Positive
        }else if(self<&T::ZERO){
            Sign::Negative
        }else{
            Sign::Zero
        }
    }
}
impl<T> HasPartialSign for T where T:AddId+PartialOrd{
    fn partial_sign(&self) -> Option<Sign> {
        match self.partial_cmp(&T::ZERO) {
            Some(Ordering::Less) => Some(Sign::Negative),
            Some(Ordering::Equal) => Some(Sign::Zero),
            Some(Ordering::Greater) => Some(Sign::Positive),
            None => None,
        }
    }
    fn is_positive(&self) -> bool{self>&T::ZERO}
    fn is_negative(&self) -> bool{self<&T::ZERO}
    fn is_zero(&self) -> bool{self==&T::ZERO}
    fn not_positive(&self) -> bool{self<&T::ZERO}
    fn not_negative(&self) -> bool{self>&T::ZERO}
    fn not_zero(&self) -> bool{self!=&T::ZERO}
}


pub trait Signum{
    fn signum(&self) -> Self;
}
///Implementation of Signum for basic types
macro_rules! impl_signum_signed {
    ($($t:ty),* $(,)?) => {
        $(
            impl Signum for $t {
                #[inline]
                fn signum(&self) -> $t {
                    <$t>::signum(*self)
                }
            }
        )*
    }
}
macro_rules! impl_signum_unsigned {
    ($($t:ty),* $(,)?) => {
        $(
            impl Signum for $t {
                #[inline]
                fn signum(&self) -> $t {
                    if *self == 0 {
                        0
                    }else{
                        1
                    }
                }
            }
        )*
    }
}
impl_signum_signed!(i8, i16, i32, i64, isize, i128, f32, f64);
impl_signum_unsigned!(u8, u16, u32, u64, usize, u128);


///Absolute Value, remove any direction
pub trait Abs{
    type Output;
    fn abs(self) -> Self::Output;
}
///implementation for Abs for basic types
macro_rules! impl_abs_signed {
    ($($t:ty),* $(,)?) => {
        $(
            impl Abs for $t {
                type Output = $t;
                #[inline]
                fn abs(self) -> $t {
                    <$t>::abs(self)
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
                #[inline]
                fn abs(self) -> $t {
                    self
                }
            }
        )*
    };
}
impl_abs_signed!(i8, i16, i32, i64, isize, i128, f32, f64);
impl_abs_unsigned!(u8, u16, u32, u64, usize, u128);


///Marker traits for signed and unsigned
/// (has positive and negative or only positive)
pub trait Signed{}
pub trait Unsigned{}
///Implementation of Signed and Unsigned for basic types
macro_rules! impl_unsigned {
    ($($t:ty),* $(,)?) => {
        $(
            impl Unsigned for $t {}
        )*
    };
}
impl_unsigned!(u8,u16,u32,u64,u128,usize);
macro_rules! impl_signed {
    ($($t:ty),* $(,)?) => {
        $(
            impl Signed for $t {}
        )*
    };
}
impl_signed!(i8,i16,i32,i64,i128,isize,f32,f64);