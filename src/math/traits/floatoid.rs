/// Trait for checking if a number is NaN.
pub trait IsNaN {
    fn is_nan(&self) -> bool;
    fn not_nan(&self) -> bool {
        !self.is_nan()
    }
}
impl<T: PartialEq> IsNaN for T {
    default fn is_nan(&self) -> bool {
        self != self
    }
    default fn not_nan(&self) -> bool {
        self == self
    }  
}

impl IsNaN for f32 {
    fn is_nan(&self) -> bool {
        f32::is_nan(*self)
    }
}
impl IsNaN for f64 {
    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }
}

/// Marker trait for types that can be NaN.
pub trait NaN {
    const NAN: Self;
}

pub trait IsPosInf {
    fn is_pos_inf(&self) -> bool;
    fn not_pos_inf(&self) -> bool {
        !self.is_pos_inf()
    }
}

pub trait IsNegInf {
    fn is_neg_inf(&self) -> bool;
    fn not_neg_inf(&self) -> bool {
        !self.is_neg_inf()
    }
}

pub trait MinPositiveValue {
    const MIN_POSITIVE: Self;
}

pub trait MaxNegativeValue {
    const MAX_NEGATIVE: Self;
}

pub trait PosInf {
    const POS_INF: Self;
}

pub trait NegInf {
    const NEG_INF: Self;
}

macro_rules! impl_float_values {
    ($($t:ty),* $(,)?) => {
        $(
            impl NaN for $t {
                const NAN: Self = Self::NAN;
            }
            impl PosInf for $t {
                const POS_INF: Self = Self::INFINITY;
            }
            impl NegInf for $t {
                const NEG_INF: Self = Self::NEG_INFINITY;
            }
            impl MinPositiveValue for $t {
                const MIN_POSITIVE: Self = Self::MIN_POSITIVE;
            }
            impl MaxNegativeValue for $t {
                const MAX_NEGATIVE: Self = -Self::MIN_POSITIVE;
            }
            impl IsPosInf for $t {
                fn is_pos_inf(&self) -> bool {
                    *self == <$t as PosInf>::POS_INF
                }
            }
            impl IsPosInf for &$t {
                fn is_pos_inf(&self) -> bool {
                    **self == <$t as PosInf>::POS_INF
                }
            }
            impl IsNegInf for $t {
                fn is_neg_inf(&self) -> bool {
                    *self == <$t as NegInf>::NEG_INF
                }
            }
            impl IsNegInf for &$t {
                fn is_neg_inf(&self) -> bool {
                    **self == <$t as NegInf>::NEG_INF
                }
            }
        )*
    };
}

macro_rules! impl_non_float_inf_checks {
    ($($t:ty),* $(,)?) => {
        $(
            impl IsPosInf for $t {
                fn is_pos_inf(&self) -> bool {
                    false
                }
            }
            impl IsPosInf for &$t {
                fn is_pos_inf(&self) -> bool {
                    false
                }
            }
            impl IsNegInf for $t {
                fn is_neg_inf(&self) -> bool {
                    false
                }
            }
            impl IsNegInf for &$t {
                fn is_neg_inf(&self) -> bool {
                    false
                }
            }
        )*
    };
}

impl_float_values!(f32, f64);
impl_non_float_inf_checks!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
