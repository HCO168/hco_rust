use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use crate::math::num_theory::gcd_euclid_iterative;
use crate::math::traits::{Abs, AddId, HasPartialSign};

#[derive(Debug, Copy, Clone, Hash)]
pub struct Fraction<T> {
    p: T, //numerator
    q: T, //denominator
}

impl<T> Fraction<T> {
    pub const fn new(numerator: T, denominator: T) -> Self {
        Self {p:numerator, q:denominator }
    }
    pub fn numerator(&self) -> &T {
        &self.p
    }
    pub fn denominator(&self) -> &T {
        &self.q
    }
}
impl<T:AddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign> Fraction<T> {
    pub fn save_new(numerator: T, denominator: T) -> Self {
        if denominator.is_zero() {
            panic!("Denominator cannot be zero.");
        }
        let gcd = gcd_euclid_iterative(numerator.clone().abs(), denominator.clone().abs());
        let mut p:T = numerator / gcd.clone();
        let mut q:T = denominator / gcd;

        if q.is_negative() {
            p = -p;
            q = -q;
        }
        Self { p, q }
    }
}
impl<T: Abs<Output = T>> Abs for Fraction<T> {
    type Output = Self;

    fn abs(self) -> Self::Output {
        Self {
            p: self.p.abs(),
            q: self.q,
        }
    }
}
impl<T:Mul<Output=T>+Add<Output=T> + Clone> Add for Fraction<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let num = self.p * other.q.clone() + other.p * self.q.clone();
        let den = self.q * other.q;
        Self::new(num, den)
    }
}

impl<T:Neg<Output = T>> Neg for Fraction<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            p: -self.p,
            q: self.q,
        }
    }
}
impl<T:Sub<Output = T>+Mul<Output=T> + Clone> Sub for Fraction<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let num = self.p * other.q.clone() - other.p * self.q.clone();
        let den = self.q * other.q;
        Self::new(num, den)
    }
}
impl<T:Mul<Output=T>> Mul for Fraction<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let num = self.p * other.p;
        let den = self.q * other.q;
        Self::new(num, den)
    }
}
impl<T:Mul<Output=T>+PartialEq+AddId> Div for Fraction<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        let num = self.p * other.q;
        let den = self.q * other.p;
        Self::new(num, den)
    }
}

impl<T: PartialOrd + AddId> PartialEq<Self> for Fraction<T> {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}
impl<T:PartialOrd+AddId> Eq for Fraction<T> {}
impl<T:PartialOrd+AddId+Mul<Output=T>+Sub<Output=T> + HasPartialSign> PartialOrd for Fraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if(self.q.is_zero()||other.q.is_zero()){
            return None;
        }
        Some(self.p.partial_cmp(&other.p).unwrap_or(std::cmp::Ordering::Equal))
    }
}
#[macro_export]
macro_rules! impl_fraction_into_primitive {
    ($($to:ty),* $(,)?) => {
        $(
            impl<T: Into<$to>> From<Fraction<T>> for $to {
                fn from(value: Fraction<T>) -> Self {
                    Into::<$to>::into(value.p) / Into::<$to>::into(value.q)
                }
            }

            impl<T: Into<$to> + Copy> From<&Fraction<T>> for $to {
                fn from(value: &Fraction<T>) -> Self {
                    Into::<$to>::into(value.p) / Into::<$to>::into(value.q)
                }
            }
        )*
    };
}

impl_fraction_into_primitive!(f64,f32);