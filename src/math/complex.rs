use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::traits::{ConstAddId, IsAddId, IsNaN, MulId, NaN};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    #[inline]
    pub const fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    #[inline]
    pub fn real(&self) -> &T {
        &self.re
    }

    #[inline]
    pub fn imag(&self) -> &T {
        &self.im
    }
}

impl<T: ConstAddId + MulId> Complex<T> {
    pub const I: Self = Self {
        re: T::ZERO,
        im: T::ONE,
    };
}

impl<T: ConstAddId> ConstAddId for Complex<T> {
    const ZERO: Self = Self {
        re: T::ZERO,
        im: T::ZERO,
    };
}

impl<T: ConstAddId + MulId> MulId for Complex<T> {
    const ONE: Self = Self {
        re: T::ONE,
        im: T::ZERO,
    };
}
/*
impl<T: AddId + PartialEq> IsAddId for Complex<T> {
    fn is_zero(&self) -> bool {
        self.re == T::ZERO && self.im == T::ZERO
    }
}

 */

impl<T: NaN> NaN for Complex<T> {
    const NAN: Self = Self {
        re: T::NAN,
        im: T::NAN,
    };
}

impl<T: IsNaN> IsNaN for Complex<T> {
    fn is_nan(&self) -> bool {
        self.re.is_nan() || self.im.is_nan()
    }
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Complex<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> Mul for Complex<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone(),
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<T> Div for Complex<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.re.clone() * rhs.re.clone() + rhs.im.clone() * rhs.im.clone();
        Self {
            re: (self.re.clone() * rhs.re.clone() + self.im.clone() * rhs.im.clone()) / denom.clone(),
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl<T> Complex<T>
where
    T: Clone + Neg<Output = T>,
{
    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re.clone(),
            im: -self.im.clone(),
        }
    }
}

impl<T> Complex<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T>,
{
    pub fn norm_sqr(&self) -> T {
        self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()
    }
}

impl<T: Display + IsAddId> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.im.is_zero() {
            write!(f, "{}", self.re)
        } else if self.re.is_zero() {
            write!(f, "{}i", self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Complex;
    use crate::math::traits::{ConstAddId, IsNaN, MulId};

    #[test]
    fn constants_are_correct() {
        assert_eq!(Complex::<i32>::ZERO, Complex::new(0, 0));
        assert_eq!(Complex::<i32>::ONE, Complex::new(1, 0));
        assert_eq!(Complex::<i32>::I, Complex::new(0, 1));
    }

    #[test]
    fn arithmetic_works() {
        let a = Complex::new(1, 2);
        let b = Complex::new(3, 4);

        assert_eq!(a + b, Complex::new(4, 6));
        assert_eq!(a * b, Complex::new(-5, 10));
        assert_eq!(a.conjugate(), Complex::new(1, -2));
        assert_eq!(a.norm_sqr(), 5);
    }

    #[test]
    fn nan_propagates() {
        let z = Complex::new(f64::NAN, 1.0);
        assert!(z.is_nan());
    }
}
