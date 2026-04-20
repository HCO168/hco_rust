/// Marker trait for types that may contain pairwise non-comparable values.
pub trait MaybeNonCmpPair: PartialEq {}

/// Marker trait for types that do not have pairwise non-comparable values.
pub trait NeverNonCmpPair: Eq {}
impl<T: Eq> NeverNonCmpPair for T {}

/// Identify whether two values are non-comparable as a pair.
pub trait IsNonCmpPair: PartialOrd {
    fn is_non_cmp_pair(&self, other: &Self) -> bool;

    fn is_cmp_pair(&self, other: &Self) -> bool {
        !self.is_non_cmp_pair(other)
    }
}

#[cfg(not(feature = "specialization"))]
impl<T: PartialOrd> IsNonCmpPair for T {
    fn is_non_cmp_pair(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_none()
    }
}

/// Marker trait for types that may contain single non-comparable values.
pub trait MaybeNonCmpValue: PartialEq {}

/// Marker trait for types that do not have single non-comparable values.
pub trait NeverNonCmpValue: Eq {}
impl<T: Eq> NeverNonCmpValue for T {}

/// Identify whether a single value is a non-comparable value.
pub trait IsNonCmpValue: PartialEq {
    fn is_non_cmp_value(&self) -> bool;

    fn is_cmp_value(&self) -> bool {
        !self.is_non_cmp_value()
    }
}

#[cfg(not(feature = "specialization"))]
impl<T: PartialEq> IsNonCmpValue for T {
    fn is_non_cmp_value(&self) -> bool {
        self != self
    }
}

impl MaybeNonCmpPair for f32 {}
impl MaybeNonCmpPair for f64 {}
impl MaybeNonCmpValue for f32 {}
impl MaybeNonCmpValue for f64 {}

#[cfg(feature = "specialization")]
impl<T: PartialOrd> IsNonCmpPair for T {
    default fn is_non_cmp_pair(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_none()
    }
}

#[cfg(feature = "specialization")]
impl<T: PartialEq> IsNonCmpValue for T {
    default fn is_non_cmp_value(&self) -> bool {
        self != self
    }
}

#[cfg(feature = "specialization")]
impl IsNonCmpPair for f32 {
    fn is_non_cmp_pair(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_none()
    }
}

#[cfg(feature = "specialization")]
impl IsNonCmpPair for f64 {
    fn is_non_cmp_pair(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_none()
    }
}

#[cfg(feature = "specialization")]
impl IsNonCmpValue for f32 {
    fn is_non_cmp_value(&self) -> bool {
        f32::is_nan(*self)
    }
}

#[cfg(feature = "specialization")]
impl IsNonCmpValue for f64 {
    fn is_non_cmp_value(&self) -> bool {
        f64::is_nan(*self)
    }
}

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
}

pub trait IsMaxValue: MaxValue + PartialEq {
    fn is_max_value(&self) -> bool
    where
        Self: Sized,
    {
        self.eq(&Self::MAX)
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
impl<T: MinValue + PartialEq> IsMinValue for T {}
impl<T: MaxValue + PartialEq> IsMaxValue for T {}

pub trait MinMax: PartialOrd + IsNonCmpValue {
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.partial_cmp(&other) {
            Some(core::cmp::Ordering::Greater) => other,
            _ => {
                if self.is_non_cmp_value() {
                    other
                } else {
                    self
                }
            }
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.partial_cmp(&other) {
            Some(core::cmp::Ordering::Less) => other,
            _ => {
                if self.is_non_cmp_value() {
                    other
                } else {
                    self
                }
            }
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        if self.is_non_cmp_value() {
            return if min.is_non_cmp_value() {
                max
            } else if max.is_non_cmp_value() {
                min
            } else {
                self
            };
        }

        if min <= max {
            if self < min {
                min
            } else if self > max {
                max
            } else {
                self
            }
        } else if self < max {
            max
        } else if self > min {
            min
        } else {
            self
        }
    }

    fn min_opt(self, other: Option<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        match other {
            Some(v) => Some(self.min(v)),
            None => None,
        }
    }

    fn max_opt(self, other: Option<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        match other {
            Some(v) => Some(self.max(v)),
            None => None,
        }
    }

    fn clamp_opt(self, min: Option<Self>, max: Option<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        match (min, max) {
            (Some(min_v), Some(max_v)) => Some(self.clamp(min_v, max_v)),
            (Some(min_v), None) => Some(self.max(min_v)),
            (None, Some(max_v)) => Some(self.min(max_v)),
            (None, None) => Some(self),
        }
    }
}

impl<T: MinMax> MinMax for Option<T> {
    fn min(self, other: Option<T>) -> Option<T> {
        match (self, other) {
            (Some(a), Some(b)) => Some(MinMax::min(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    fn max(self, other: Option<T>) -> Option<T> {
        match (self, other) {
            (Some(a), Some(b)) => Some(MinMax::max(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    fn clamp(self, min: Option<T>, max: Option<T>) -> Option<T> {
        match self {
            Some(v) => MinMax::clamp_opt(v, min, max),
            None => match (min, max) {
                (Some(min_v), None) => Some(min_v),
                (None, Some(max_v)) => Some(max_v),
                _ => None,
            },
        }
    }
}

macro_rules! impl_min_max_ord {
    ($($t:ty),* $(,)?) => {
        $(
            impl MinMax for $t {
                fn min(self, other: Self) -> Self {
                    if self < other { self } else { other }
                }

                fn max(self, other: Self) -> Self {
                    if self > other { self } else { other }
                }

                fn clamp(self, min: Self, max: Self) -> Self {
                    if min <= max {
                        if self < min {
                            min
                        } else if self > max {
                            max
                        } else {
                            self
                        }
                    } else if self < max {
                        max
                    } else if self > min {
                        min
                    } else {
                        self
                    }
                }
            }

            impl MinMax for &$t {
                fn min(self, other: Self) -> Self {
                    if self < other { self } else { other }
                }

                fn max(self, other: Self) -> Self {
                    if self > other { self } else { other }
                }

                fn clamp(self, min: Self, max: Self) -> Self {
                    if min <= max {
                        if self < min {
                            min
                        } else if self > max {
                            max
                        } else {
                            self
                        }
                    } else if self < max {
                        max
                    } else if self > min {
                        min
                    } else {
                        self
                    }
                }
            }
        )*
    };
}

macro_rules! impl_min_max_float {
    ($($t:ty),* $(,)?) => {
        $(
            impl MinMax for $t {
                fn min(self, other: Self) -> Self {
                    <$t>::min(self, other)
                }

                fn max(self, other: Self) -> Self {
                    <$t>::max(self, other)
                }
            }

            impl MinMax for &$t {
                fn min(self, other: Self) -> Self {
                    if *self < *other { self } else { other }
                }

                fn max(self, other: Self) -> Self {
                    if *self > *other { self } else { other }
                }
            }
        )*
    };
}

impl_min_max_ord!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_min_max_float!(f32, f64);

#[macro_export]
macro_rules! min {
    ($first:expr $(,)?) => {{
        $first
    }};
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let mut acc = $first;
        $(
            acc = $crate::math::traits::cmp::MinMax::min(acc, $rest);
        )+
        acc
    }};
}

#[macro_export]
macro_rules! max {
    ($first:expr $(,)?) => {{
        $first
    }};
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let mut acc = $first;
        $(
            acc = $crate::math::traits::cmp::MinMax::max(acc, $rest);
        )+
        acc
    }};
}
#[cfg(test)]
mod test{
    fn test_min_macro_0(){
        assert_eq!(min!(3.0f32, 2.0, 5.0, 1.0), 1.0);
    }
    fn test_max_macro_0(){
        assert_eq!(max!(3.0f32, 2.0, 5.0, 1.0), 5.0);
    }
}