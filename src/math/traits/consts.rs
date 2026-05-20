

pub trait MinValue {
    const MIN: Self;
}

pub trait MaxValue {
    const MAX: Self;
}

pub trait IsMinValue: MinValue + PartialEq {
    fn is_min_value(&self) -> bool
    where
        Self: Sized,
    {
        self.eq(&Self::MIN)
    }

    fn not_min_value(&self) -> bool
    where
        Self: Sized,
    {
        !self.is_min_value()
    }
}

pub trait IsMaxValue: MaxValue + PartialEq {
    fn is_max_value(&self) -> bool
    where
        Self: Sized,
    {
        self.eq(&Self::MAX)
    }

    fn not_max_value(&self) -> bool
    where
        Self: Sized,
    {
        !self.is_max_value()
    }
}

macro_rules! impl_min_max_value {
    ($($t:ty),* $(,)?) => {
        $(
            impl MinValue for $t {
                const MIN: Self = <$t>::MIN;
            }

            impl MaxValue for $t {
                const MAX: Self = <$t>::MAX;
            }
        )*
    };
}

impl_min_max_value!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
#[cfg(feature = "specialization")]
impl<T: MinValue + PartialEq> IsMinValue for T {}
#[cfg(feature = "specialization")]
impl<T: MaxValue + PartialEq> IsMaxValue for T {}


pub trait MinFiniteValue {
    const MIN_FINITE: Self;
}

pub trait MaxFiniteValue {
    const MAX_FINITE: Self;
}

pub trait IsMinFiniteValue: MinFiniteValue + PartialEq {
    fn is_min_finite_value(&self) -> bool
    where
        Self: Sized,
    {
        self.eq(&Self::MIN_FINITE)
    }

    fn not_min_finite_value(&self) -> bool
    where
        Self: Sized,
    {
        !self.is_min_finite_value()
    }
}

pub trait IsMaxFiniteValue: MaxFiniteValue + PartialEq {
    fn is_max_finite_value(&self) -> bool
    where
        Self: Sized,
    {
        self.eq(&Self::MAX_FINITE)
    }

    fn not_max_finite_value(&self) -> bool
    where
        Self: Sized,
    {
        !self.is_max_finite_value()
    }
}

pub trait NaN {
    const NAN: Self;
}

pub trait PosInf {
    const POS_INF: Self;
}

pub trait NegInf {
    const NEG_INF: Self;
}

pub trait IsPosInf {
    fn is_pos_inf(&self) -> bool {
        false
    }

    fn not_pos_inf(&self) -> bool {
        !self.is_pos_inf()
    }
}

pub trait IsNegInf {
    fn is_neg_inf(&self) -> bool {
        false
    }

    fn not_neg_inf(&self) -> bool {
        !self.is_neg_inf()
    }
}
pub trait IsInf: IsPosInf + IsNegInf {
    fn is_inf(&self) -> bool{
        self.is_pos_inf()||self.is_neg_inf()
    }
    fn not_inf(&self) -> bool {
        !self.is_inf()
    }
}

pub trait MinPositiveValue {
    const MIN_POSITIVE: Self;
}

pub trait MaxNegativeValue {
    const MAX_NEGATIVE: Self;
}

pub trait IsMinPositiveValue {
    fn is_min_positive_value(&self) -> bool {
        false
    }

    fn not_min_positive_value(&self) -> bool {
        !self.is_min_positive_value()
    }
}

pub trait IsMaxNegativeValue {
    fn is_max_negative_value(&self) -> bool {
        false
    }

    fn not_max_negative_value(&self) -> bool {
        !self.is_max_negative_value()
    }
}

#[cfg(feature = "specialization")]
impl<T: MinValue> MinFiniteValue for T {
    const MIN_FINITE: Self = T::MIN;
}

#[cfg(feature = "specialization")]
impl<T: MaxValue> MaxFiniteValue for T {
    const MAX_FINITE: Self = T::MAX;
}

#[cfg(feature = "specialization")]
impl<T: MinFiniteValue + PartialEq> IsMinFiniteValue for T {}
#[cfg(feature = "specialization")]
impl<T: MaxFiniteValue + PartialEq> IsMaxFiniteValue for T {}

macro_rules! impl_finite_value_checks {
    ($($t:ty),* $(,)?) => {
        $(
            #[cfg(not(feature = "specialization"))]
            impl IsMinValue for $t {}

            #[cfg(not(feature = "specialization"))]
            impl IsMaxValue for $t {}

            #[cfg(not(feature = "specialization"))]
            impl MinFiniteValue for $t {
                const MIN_FINITE: Self = <$t>::MIN;
            }

            #[cfg(not(feature = "specialization"))]
            impl MaxFiniteValue for $t {
                const MAX_FINITE: Self = <$t>::MAX;
            }

            #[cfg(not(feature = "specialization"))]
            impl IsMinFiniteValue for $t {}

            #[cfg(not(feature = "specialization"))]
            impl IsMaxFiniteValue for $t {}
        )*
    };
}

impl_finite_value_checks!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl NaN for f32 {
    const NAN: Self = f32::NAN;
}

impl NaN for f64 {
    const NAN: Self = f64::NAN;
}

impl PosInf for f32 {
    const POS_INF: Self = f32::INFINITY;
}

impl PosInf for f64 {
    const POS_INF: Self = f64::INFINITY;
}

impl NegInf for f32 {
    const NEG_INF: Self = f32::NEG_INFINITY;
}

impl NegInf for f64 {
    const NEG_INF: Self = f64::NEG_INFINITY;
}

impl MinPositiveValue for f32 {
    const MIN_POSITIVE: Self = f32::MIN_POSITIVE;
}

impl MinPositiveValue for f64 {
    const MIN_POSITIVE: Self = f64::MIN_POSITIVE;
}

impl MaxNegativeValue for f32 {
    const MAX_NEGATIVE: Self = -f32::MIN_POSITIVE;
}

impl MaxNegativeValue for f64 {
    const MAX_NEGATIVE: Self = -f64::MIN_POSITIVE;
}

#[cfg(feature = "specialization")]
impl<T: PartialEq + PosInf> IsPosInf for T {
    fn is_pos_inf(&self) -> bool {
        self == &Self::POS_INF
    }
}

#[cfg(feature = "specialization")]
impl<T: PartialEq + NegInf> IsNegInf for T {
    fn is_neg_inf(&self) -> bool {
        self == &Self::NEG_INF
    }
}
#[cfg(feature = "specialization")]
impl<T:IsPosInf + IsNegInf> IsInf for T {
    fn is_inf(&self) -> bool {
        self.is_pos_inf() || self.is_neg_inf()
    }
}
#[cfg(feature = "specialization")]
impl<T: PartialEq + MinPositiveValue> IsMinPositiveValue for T {
    fn is_min_positive_value(&self) -> bool {
        self == &Self::MIN_POSITIVE
    }
}

#[cfg(feature = "specialization")]
impl<T: PartialEq + MaxNegativeValue> IsMaxNegativeValue for T {
    fn is_max_negative_value(&self) -> bool {
        self == &Self::MAX_NEGATIVE
    }
}

macro_rules! impl_false_checks {
    ($($t:ty),* $(,)?) => {
        $(
            impl IsPosInf for $t {}

            impl IsNegInf for $t {}

            #[cfg(not(feature = "specialization"))]
            impl IsInf for $t {}

            impl IsMinPositiveValue for $t {}

            impl IsMaxNegativeValue for $t {}
        )*
    };
}

impl_false_checks!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, bool, char);

macro_rules! impl_float_inf_checks {
    ($($t:ty),* $(,)?) => {
        $(
            #[cfg(not(feature = "specialization"))]
            impl IsPosInf for $t {
                fn is_pos_inf(&self) -> bool {
                    self == &Self::POS_INF
                }
            }

            #[cfg(not(feature = "specialization"))]
            impl IsNegInf for $t {
                fn is_neg_inf(&self) -> bool {
                    self == &Self::NEG_INF
                }
            }

            #[cfg(not(feature = "specialization"))]
            impl IsInf for $t {}

            #[cfg(not(feature = "specialization"))]
            impl IsMinPositiveValue for $t {
                fn is_min_positive_value(&self) -> bool {
                    self == &Self::MIN_POSITIVE
                }
            }

            #[cfg(not(feature = "specialization"))]
            impl IsMaxNegativeValue for $t {
                fn is_max_negative_value(&self) -> bool {
                    self == &Self::MAX_NEGATIVE
                }
            }
        )*
    };
}

impl_float_inf_checks!(f32, f64);
