use std::ops::Rem;
use crate::math::traits::{Abs, AddId, HasPartialSign};

pub fn gcd_euclid_recursive<T>(a: T, b: T) -> T
    where
        T:AddId+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone + HasPartialSign
{
    let a = a.abs();
    let b = b.abs();
    if b.is_zero() {
        return a;
    }
    gcd_euclid_recursive(b.clone(), a % b)
}
pub fn gcd_euclid_iterative<T>(mut a: T, mut b: T) -> T
    where
        T:AddId+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone + HasPartialSign
{
    a = a.abs();
    b = b.abs();
    while b.not_zero() {
        let r:T = a % b.clone();
        a = b;
        b = r;
    }
    a
}

pub const fn gcd_stein(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b.abs();
    }
    if b == 0 {
        return a.abs();
    }

    let shift = (a | b).trailing_zeros();

    a = a.abs();
    b = b.abs();

    a >>= a.trailing_zeros();
    while b != 0 {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
    }

    a << shift
}

pub const fn gcd_subtractive(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

#[inline]
pub const fn gcd(a: i64, b: i64) -> i64 {
    // Keep the default fast implementation here.
    gcd_stein(a, b)
}
