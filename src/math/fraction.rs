use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use crate::math::{IsAddId, NaN, NegInf, NegMulId, PosInf, GCD};
use crate::math::num_theory::gcd_euclid_iterative;
use crate::math::traits::{Abs, AddId, ConstAddId, HasPartialSign,IsNaN, Signum,MulId};

#[derive(Debug, Copy, Clone, Hash)]
pub struct Fraction<T> {
    p: T, //numerator
    q: T, //denominator
}
impl<T> Fraction<T> {
    #[inline]
    pub fn numerator(&self) -> &T {
        &self.p
    }
    #[inline]
    pub fn denominator(&self) -> &T {
        &self.q
    }
    /** Using this function may make calculations slower significantly since it does not simplify the fraction.*/
    pub const fn raw_new(numerator: T, denominator: T) -> Self {
        Self { p:numerator, q:denominator }
    }
}
impl<T:AddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign+Signum+GCD>
Fraction<T> {
    pub fn new(numerator: T, denominator: T) -> Self {
        if denominator.is_zero() {
            // Follow floating-like semantics:
            // - non-zero / 0 => +/-Inf (sign comes from numerator)
            // - 0 / 0 => NaN
            // any argument is NaN => NaN
            return Self { p: numerator.signum(), q: denominator};
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
impl<T: Abs<Output = T>>
Abs for Fraction<T> {
    type Output = Self;

    fn abs(self) -> Self::Output {
        Self {
            p: self.p.abs(),
            q: self.q,
        }
    }
}
impl<T:ConstAddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign+Mul<Output=T>+Add<Output=T>+Signum+GCD>
Add for Fraction<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if(self.is_nan() || other.is_nan()){
            return Self::NAN;
        }
        if(self.q==other.q){
            return Self::new(self.p+other.p,self.q);
        }
        let num = self.p * other.q.clone() + other.p * self.q.clone();
        let den = self.q * other.q;
        Self::new(num, den)
    }
}

impl<T:Neg<Output = T>>
Neg for Fraction<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            p: -self.p,
            q: self.q,
        }
    }
}
impl<T:ConstAddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign+Add<Output=T>+Mul<Output=T>+Signum+GCD>
Sub for Fraction<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.add(-other)
    }
}
impl<T:AddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign+Mul<Output=T>+Signum+GCD>
Mul for Fraction<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let num = self.p * other.p;
        let den = self.q * other.q;
        Self::new(num, den)
    }
}
impl<T:AddId+Neg<Output=T>+Abs<Output=T>+PartialOrd+Rem<Output = T>+Clone+Div<Output = T> + HasPartialSign+Mul<Output=T>+PartialEq+Signum+GCD>
Div for Fraction<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self * Self::new(other.q, other.p)
    }
}

impl<T: PartialEq + IsAddId>
PartialEq<Self> for Fraction<T> {
    fn eq(&self, other: &Self) -> bool {
        // NaN semantics: NaN != NaN
        if self.is_nan() || other.is_nan() {
            return false;
        }
        self.p == other.p && self.q == other.q
    }
}
impl<T: PartialOrd + Mul<Output = T> + Clone + HasPartialSign>
PartialOrd for Fraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if(self.q.is_zero()||other.q.is_zero()){
            // NaN is unordered with everything.
            if (self.q.is_zero() && self.p.is_zero()) || (other.q.is_zero() && other.p.is_zero()) {
                return None;
            }
            // Infinity ordering.
            if self.q.is_zero() && other.q.is_zero() {
                return self.p.partial_cmp(&other.p);
            }
            if other.q.is_zero() {
                return if other.p.is_positive() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                };
            }
        }
        let left = self.p.clone() * other.q.clone();
        let right = other.p.clone() * self.q.clone();
        left.partial_cmp(&right)
    }
}
impl<T:ConstAddId+MulId> ConstAddId for Fraction<T> {
    const ZERO: Self =Self { p: T::ZERO, q: T::ONE };
}
impl<T:ConstAddId+MulId+PartialEq+IsAddId> IsAddId for Fraction<T> {
    fn is_zero(&self) -> bool {
        self.p.is_zero() && self.q.not_zero()
    }
}
impl<T:MulId> MulId for Fraction<T> {
    const ONE: Self =Self { p: T::ONE, q: T::ONE };
}
impl<T:ConstAddId> NaN for Fraction<T> {
    const NAN: Self = Self{p: T::ZERO, q: T::ZERO};
}
impl<T:PartialEq+IsAddId> IsNaN for Fraction<T> {
    fn is_nan(&self) -> bool {
        self.q.is_zero() && self.p.is_zero()
    }
}
impl<T:ConstAddId+MulId> PosInf for Fraction<T> {
    const POS_INF: Self = Self{p: T::ONE, q: T::ZERO};
}
impl<T:ConstAddId+MulId+NegMulId> NegInf for Fraction<T> {
    const NEG_INF: Self = Self{p: T::NEG_ONE, q: T::ZERO};
}
impl<T:Display> Display for Fraction<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.p, self.q))
    }
}
impl Fraction<i64> {
    /// Java `constructFromDouble` style conversion:
    /// decompose IEEE-754 bits, keep binary structure, and bound denominator shifts.
    pub fn from_f64_binary(m: f64) -> Self {
        if m == 0.0 {
            return Self::raw_new(0, 1);
        }
        if !m.is_finite() {
            panic!("Input used to construct Fraction<i64> from f64 must be finite. input={m}");
        }

        let bits = m.to_bits();
        let sign_neg = (bits >> 63) != 0;
        let exp_bits = ((bits >> 52) & 0x7ff) as i32;
        let frac_bits = bits & ((1u64 << 52) - 1);

        // significand (includes implicit leading 1 for normal numbers)
        let mut p = if exp_bits == 0 {
            frac_bits as i64
        } else {
            ((1u64 << 52) | frac_bits) as i64
        };
        if p == 0 {
            return Self::raw_new(0, 1);
        }

        // remove trailing binary zeros from significand
        p >>= p.trailing_zeros();

        // unbiased exponent
        let log2_result = if exp_bits == 0 { -1022 } else { exp_bits - 1023 };
        let n = 64 - p.unsigned_abs().leading_zeros() as i32;
        let mut shift = log2_result - n + 1;
        let mut q: i64 = 1;

        if shift > 0 {
            // guard numerator overflow
            if shift > 63 - n {
                panic!("Input too large for Fraction<i64> numerator. input={m}");
            }
            p <<= shift;
        } else if shift < 0 {
            // bound denominator to 2^62; for tiny numbers, sacrifice precision like Java code
            if shift < -62 {
                p >>= -shift - 62;
                if p == 0 {
                    return Self::raw_new(0, 1);
                }
                shift = -62;
            }
            q <<= -shift;
        }

        if sign_neg {
            p = -p;
        }
        Self::new(p, q)
    }
}

impl From<f64> for Fraction<i64> {
    fn from(value: f64) -> Self {
        Fraction::<i64>::from_f64_binary(value)
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
#[cfg(test)]
mod test{
    use std::f64::NAN;
    use crate::math::{Fraction, HasPartialSign, HasSign, IsInf, NegInf, PosInf};

    const TEST_VALUE_F64:[f64;6]=[f64::NEG_INF,-1.0,0.0,1.0,f64::POS_INF,NAN];
    const TEST_VALUE_FRACTION_I64:[Fraction<i64>;6]=
        [Fraction::raw_new(-1,0),Fraction::raw_new(-1,1),Fraction::raw_new(0,1),
        Fraction::raw_new(1,1),Fraction::raw_new(1,0),Fraction::raw_new(0,0)];
    fn check_equivalent(n:f64,f:Fraction<i64>)->bool{
        n.partial_sign()==f.partial_sign()
        &&n.is_infinite()==f.is_inf()
    }
    #[test]
    fn test_fraction_floatiod_add_behavior() {
        for i in 0..6 {
            for j in 0..6 {
                let n1=TEST_VALUE_F64[i];
                let n2=TEST_VALUE_F64[j];
                let f1=TEST_VALUE_FRACTION_I64[i];
                let f2=TEST_VALUE_FRACTION_I64[j];

                let n_add_result=n1+n2;
                let f_add_result=f1+f2;

                assert_eq!(
                    (n_add_result.partial_sign(), n_add_result.is_infinite()),
                    (f_add_result.partial_sign(), f_add_result.is_inf()),
                    "add behavior mismatch: n1={}, n2={}, f1={}, f2={}, n1+n2={}, f1+f2={}, n_sign={:?}, f_sign={:?}, n_inf={}, f_inf={}",
                    n1,n2,f1,f2,n_add_result,f_add_result,
                    n_add_result.partial_sign(),f_add_result.partial_sign(),
                    n_add_result.is_infinite(),f_add_result.is_inf()
                );
            }
        }
    }

    #[test]
    fn test_fraction_floatiod_sub_behavior() {
        for i in 0..6 {
            for j in 0..6 {
                let n1=TEST_VALUE_F64[i];
                let n2=TEST_VALUE_F64[j];
                let f1=TEST_VALUE_FRACTION_I64[i];
                let f2=TEST_VALUE_FRACTION_I64[j];

                let n_sub_result=n1-n2;
                let f_sub_result=f1-f2;

                assert_eq!(
                    (n_sub_result.partial_sign(), n_sub_result.is_infinite()),
                    (f_sub_result.partial_sign(), f_sub_result.is_inf()),
                    "sub behavior mismatch: n1={}, n2={}, f1={}, f2={}, n1-n2={}, f1-f2={}, n_sign={:?}, f_sign={:?}, n_inf={}, f_inf={}",
                    n1,n2,f1,f2,n_sub_result,f_sub_result,
                    n_sub_result.partial_sign(),f_sub_result.partial_sign(),
                    n_sub_result.is_infinite(),f_sub_result.is_inf()
                );
            }
        }
    }

    #[test]
    fn test_fraction_floatiod_mul_behavior() {
        for i in 0..6 {
            for j in 0..6 {
                let n1=TEST_VALUE_F64[i];
                let n2=TEST_VALUE_F64[j];
                let f1=TEST_VALUE_FRACTION_I64[i];
                let f2=TEST_VALUE_FRACTION_I64[j];

                let n_mult_result=n1*n2;
                let f_mult_result=f1*f2;

                assert_eq!(
                    (n_mult_result.partial_sign(), n_mult_result.is_infinite()),
                    (f_mult_result.partial_sign(), f_mult_result.is_inf()),
                    "mul behavior mismatch: n1={}, n2={}, f1={}, f2={}, n1*n2={}, f1*f2={}, n_sign={:?}, f_sign={:?}, n_inf={}, f_inf={}",
                    n1,n2,f1,f2,n_mult_result,f_mult_result,
                    n_mult_result.partial_sign(),f_mult_result.partial_sign(),
                    n_mult_result.is_infinite(),f_mult_result.is_inf()
                );
            }
        }
    }

    #[test]
    fn test_fraction_floatiod_div_behavior() {
        for i in 0..6 {
            for j in 0..6 {
                let n1=TEST_VALUE_F64[i];
                let n2=TEST_VALUE_F64[j];
                let f1=TEST_VALUE_FRACTION_I64[i];
                let f2=TEST_VALUE_FRACTION_I64[j];

                let n_div_result=n1/n2;
                let f_div_result=f1/f2;

                assert_eq!(
                    (n_div_result.partial_sign(), n_div_result.is_infinite()),
                    (f_div_result.partial_sign(), f_div_result.is_inf()),
                    "div behavior mismatch: n1={}, n2={}, f1={}, f2={}, n1/n2={}, f1/f2={}, n_sign={:?}, f_sign={:?}, n_inf={}, f_inf={}",
                    n1,n2,f1,f2,n_div_result,f_div_result,
                    n_div_result.partial_sign(),f_div_result.partial_sign(),
                    n_div_result.is_infinite(),f_div_result.is_inf()
                );
            }
        }
    }

    #[test]
    fn test_fraction_floatiod_cmp_behavior() {
        for i in 0..6 {
            for j in 0..6 {
                let n1=TEST_VALUE_F64[i];
                let n2=TEST_VALUE_F64[j];
                let f1=TEST_VALUE_FRACTION_I64[i];
                let f2=TEST_VALUE_FRACTION_I64[j];

                assert_eq!(
                    n1.partial_cmp(&n2),
                    f1.partial_cmp(&f2),
                    "partial_cmp mismatch: n1={}, n2={}, f1={}, f2={}, n_cmp={:?}, f_cmp={:?}",
                    n1,n2,f1,f2,n1.partial_cmp(&n2),f1.partial_cmp(&f2)
                );
            }
        }
    }
    #[test]
    fn test_fraction_behave_like_double(){
        const EPSILON: f64 = 1e-12;

        fn to_f64(fraction: Fraction<i64>) -> f64 {
            *fraction.numerator() as f64 / *fraction.denominator() as f64
        }

        fn float_eq(left: f64, right: f64) -> bool {
            (left - right).abs() <= EPSILON
        }

        let fractions = [
            Fraction::new(-7_i64, 3),
            Fraction::new(-5_i64, 2),
            Fraction::new(-2_i64, 5),
            Fraction::new(0_i64, 1),
            Fraction::new(1_i64, 3),
            Fraction::new(1_i64, 2),
            Fraction::new(2_i64, 5),
            Fraction::new(7_i64, 10),
            Fraction::new(4_i64, 3),
            Fraction::new(9_i64, 4),
        ];

        fn assert_same_as_double(
            lhs: Fraction<i64>,
            rhs: Fraction<i64>,
            op_name: &str,
            fraction_result: Fraction<i64>,
            double_result: f64,
        ) {
            let fraction_then_double = to_f64(fraction_result);
            assert!(
                float_eq(fraction_then_double, double_result),
                "{} mismatch: lhs={}, rhs={}, (lhs {} rhs) as f64={}, lhs_f64 {} rhs_f64={}",
                op_name, lhs, rhs, op_name, fraction_then_double, op_name, double_result
            );
        }

        for lhs in fractions {
            for rhs in fractions {
                let lhs_f64 = to_f64(lhs);
                let rhs_f64 = to_f64(rhs);

                assert_same_as_double(lhs, rhs, "+", lhs + rhs, lhs_f64 + rhs_f64);
                assert_same_as_double(lhs, rhs, "-", lhs - rhs, lhs_f64 - rhs_f64);
                assert_same_as_double(lhs, rhs, "*", lhs * rhs, lhs_f64 * rhs_f64);

                if rhs != Fraction::new(0_i64, 1) {
                    assert_same_as_double(lhs, rhs, "/", lhs / rhs, lhs_f64 / rhs_f64);
                }
            }
        }
    }
}
