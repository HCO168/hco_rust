use crate::math::math::gcd_stein;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fraction {
    p: i64,
    q: i64,
}

impl Fraction {
    pub const fn new(numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("Denominator cannot be zero.");
        }

        let gcd = gcd_stein(numerator.abs(), denominator.abs());
        let mut num = numerator / gcd;
        let mut den = denominator / gcd;

        if den < 0 {
            num = -num;
            den = -den;
        }

        Self { p: num, q: den }
    }

    pub fn numerator(&self) -> i64 {
        self.p
    }

    pub fn denominator(&self) -> i64 {
        self.q
    }

    pub fn abs(&self) -> Self {
        Self {
            p: self.p.abs(),
            q: self.q,
        }
    }

    pub fn negate(&self) -> Self {
        Self {
            p: -self.p,
            q: self.q,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let num = self.p * other.q + other.p * self.q;
        let den = self.q * other.q;
        Self::new(num, den)
    }

    pub fn subtract(&self, other: &Self) -> Self {
        let num = self.p * other.q - other.p * self.q;
        let den = self.q * other.q;
        Self::new(num, den)
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let num = self.p * other.p;
        let den = self.q * other.q;
        Self::new(num, den)
    }

    pub fn divide(&self, other: &Self) -> Self {
        if other.p == 0 {
            panic!("Cannot divide by zero fraction.");
        }
        let num = self.p * other.q;
        let den = self.q * other.p;
        Self::new(num, den)
    }

    pub fn to_f64(&self) -> f64 {
        self.p as f64 / self.q as f64
    }

    pub fn to_i64(&self) -> i64 {
        self.p / self.q
    }
}
