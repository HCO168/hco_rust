
pub trait NAN {
    fn is_nan(&self) -> bool;
}
impl<T: PartialEq> NAN for T {
    fn is_nan(&self) -> bool {
        self != self
    }
}
#[repr(i8)]
pub enum Sign{
    Positive=1,
    Zero=0,
    Negative=-1,
    NaN=-128,
}
impl Sign{
    pub const fn to_i8(self) -> i8 {
        self as i8
    }
    pub const fn is_positive(&self) -> bool {
        match self {
            Sign::Positive => true,
            _ => false,
        }
    }
    pub const fn is_zero(&self) -> bool {
        match self {
            Sign::Zero => true,
            _ => false,
        }
    }
    pub const fn is_negative(&self) -> bool {
        match self {
            Sign::Negative => true,
            _ => false,
        }
    }
    pub const fn not_positive(&self) -> bool {
        match self {
            Sign::Positive => false,
            _ => true,
        }
    }
    pub const fn not_zero(&self) -> bool {
        match self {
            Sign::Zero => false,
            _ => true,
        }
    }
    pub const fn not_negative(&self) -> bool {
        match self{
            Sign::Negative => false,
            _ => true,
        }
    }
}
pub trait Signed{
    fn sign(&self) -> Sign;
}