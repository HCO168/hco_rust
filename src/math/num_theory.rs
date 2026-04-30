use std::ops::{BitOr, Rem, Shl, ShrAssign, SubAssign};
use crate::math::{IsAddId, TrailingZeros};
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

pub fn gcd_stein<T>(mut a: T, mut b: T) -> T
where T:IsAddId+PartialOrd+SubAssign+Abs<Output = T> +BitOr<Output=T> +TrailingZeros +ShrAssign<usize>+Copy +Shl<usize, Output = T>{
    if a.is_zero(){
        return b.abs();
    }
    if b.is_zero() {
        return a.abs();
    }

    let shift = (a | b).trailing_zeros();

    a = a.abs();
    b = b.abs();

    a >>= a.trailing_zeros();
    while b.not_zero() {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
    }

    a << shift
}

pub fn gcd_subtractive<T>(mut a: T, mut b: T) -> T
where T:AddId+PartialOrd+SubAssign+Abs<Output = T> +HasPartialSign + Clone{
    a = a.abs();
    b = b.abs();
    if a.is_zero() {
        return b;
    }
    if b.is_zero(){
        return a;
    }

    while a != b {
        if a > b {
            a -= b.clone();
        } else {
            b -= a.clone();
        }
    }
    a
}
