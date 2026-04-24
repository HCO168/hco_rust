use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use crate::math::num_theory::gcd_euclid_iterative;
use crate::math::traits::{Abs, AddId, HasPartialSign};

#[derive(Debug, Copy, Clone, Hash)]
pub struct Fraction<T> {
    p: T, //numerator
    q: T, //denominator
}

impl<T> Fraction<T> {
    pub fn numerator(&self) -> &T {
        &self.p
    }
    pub fn denominator(&self) -> &T {
        &self.q
    }
}
impl<T:AddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign> Fraction<T> {
    pub fn new(numerator: T, denominator: T) -> Self {
        Self::save_new(numerator, denominator)
    }

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
impl<T:AddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign+Mul<Output=T>+Add<Output=T>> Add for Fraction<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let num = self.p * other.q.clone() + other.p * self.q.clone();
        let den = self.q * other.q;
        Self::save_new(num, den)
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
impl<T:AddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign+Sub<Output = T>+Mul<Output=T>> Sub for Fraction<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let num = self.p * other.q.clone() - other.p * self.q.clone();
        let den = self.q * other.q;
        Self::save_new(num, den)
    }
}
impl<T:AddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign+Mul<Output=T>> Mul for Fraction<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let num = self.p * other.p;
        let den = self.q * other.q;
        Self::save_new(num, den)
    }
}
impl<T:AddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign+Mul<Output=T>+PartialEq> Div for Fraction<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        if other.p == T::ZERO {
            panic!("Division by zero fraction.");
        }
        let num = self.p * other.q;
        let den = self.q * other.p;
        Self::save_new(num, den)
    }
}

impl<T: PartialEq> PartialEq<Self> for Fraction<T> {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p && self.q == other.q
    }
}
impl<T: Eq> Eq for Fraction<T> {}
impl<T: PartialOrd + Mul<Output = T> + Clone + HasPartialSign> PartialOrd for Fraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.q.is_zero() || other.q.is_zero() {
            return None;
        }
        let left = self.p.clone() * other.q.clone();
        let right = other.p.clone() * self.q.clone();
        left.partial_cmp(&right)
    }
}
impl<T:Display> Display for Fraction<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.p, self.q))
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
