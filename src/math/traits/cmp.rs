/// Marker trait for types that may contain pairwise non-comparable values.
pub trait MaybeNonCmpPair: PartialOrd {}

/// Marker trait for types that do not have pairwise non-comparable values.
pub trait NeverNonCmpPair: Ord {}
#[cfg(feature = "specialization")]
impl<T: Ord> NeverNonCmpPair for T {}

/// Identify whether two values are non-comparable as a pair.
pub trait IsNonCmpPair: PartialOrd {
    fn is_non_cmp_pair(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_none()
    }

    fn is_cmp_pair(&self, other: &Self) -> bool {
        !self.is_non_cmp_pair(other)
    }
}

#[cfg(feature = "specialization")]
impl<T: PartialOrd> IsNonCmpPair for T {}

/// Marker trait for types that may contain single non-comparable values.
pub trait MaybeNonCmpValue: PartialEq {}

/// Marker trait for types that do not have single non-comparable values.
pub trait NeverNonCmpValue: Eq {}
#[cfg(feature = "specialization")]
impl<T: Eq> NeverNonCmpValue for T {}

/// Identify whether a single value is a non-comparable value.
pub trait IsNonCmpValue: PartialEq {
    fn is_non_cmp_value(&self) -> bool {
        self != self
    }

    fn is_cmp_value(&self) -> bool {
        !self.is_non_cmp_value()
    }
}

#[cfg(feature = "specialization")]
impl<T: PartialEq> IsNonCmpValue for T {}

macro_rules! impl_cmp_traits_for_primitives {
    ($($t:ty),* $(,)?) => {
        $(
            #[cfg(not(feature = "specialization"))]
            impl NeverNonCmpPair for $t {}

            #[cfg(not(feature = "specialization"))]
            impl IsNonCmpPair for $t {}

            #[cfg(not(feature = "specialization"))]
            impl NeverNonCmpValue for $t {}

            #[cfg(not(feature = "specialization"))]
            impl IsNonCmpValue for $t {}

            #[cfg(not(feature = "specialization"))]
            impl IsNonCmpPair for &$t {}

            #[cfg(not(feature = "specialization"))]
            impl IsNonCmpValue for &$t {}
        )*
    };
}

macro_rules! impl_float_cmp_traits {
    ($($t:ty),* $(,)?) => {
        $(
            #[cfg(not(feature = "specialization"))]
            impl IsNonCmpPair for $t {}

            #[cfg(not(feature = "specialization"))]
            impl IsNonCmpValue for $t {}

            #[cfg(not(feature = "specialization"))]
            impl IsNonCmpPair for &$t {}

            #[cfg(not(feature = "specialization"))]
            impl IsNonCmpValue for &$t {}
        )*
    };
}

impl_cmp_traits_for_primitives!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, bool, char);
impl_float_cmp_traits!(f32, f64);

#[cfg(not(feature = "specialization"))]
impl<T: PartialEq> IsNonCmpValue for Option<T> {}

#[cfg(not(feature = "specialization"))]
impl<T: PartialOrd> IsNonCmpPair for Option<T> {}

impl MaybeNonCmpPair for f32 {}
impl MaybeNonCmpPair for f64 {}
impl MaybeNonCmpValue for f32 {}
impl MaybeNonCmpValue for f64 {}
#[inline(always)]
fn ignore_non_cmp_values<T: IsNonCmpValue>(a: T,b: T) -> T {
    if a.is_non_cmp_value() {
        b
    } else {
        a
    }
}
pub trait MinMax: PartialOrd + IsNonCmpValue {
    fn min_num(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.partial_cmp(&other) {
            Some(core::cmp::Ordering::Greater) => other,
            _ => {
                ignore_non_cmp_values(self, other)
            }
        }
    }
    fn max_num(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.partial_cmp(&other) {
            Some(core::cmp::Ordering::Less) => other,
            _ => {
                ignore_non_cmp_values(self, other)
            }
        }
    }
    fn clamp_num(self, min: Self, max: Self) -> Self
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

    fn minimum(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.is_non_cmp_value() {
            self
        } else if other.is_non_cmp_value() {
            other
        } else {
            self.min_num(other)
        }
    }

    fn maximum(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.is_non_cmp_value() {
            self
        } else if other.is_non_cmp_value() {
            other
        } else {
            self.max_num(other)
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        if self.is_non_cmp_value() {
            self
        } else if min.is_non_cmp_value() {
            min
        } else if max.is_non_cmp_value() {
            max
        } else {
            self.clamp_num(min, max)
        }
    }
}

fn option_zip_some<T>(lhs: Option<T>, rhs: Option<T>, f: fn(T, T) -> T) -> Option<T> {
    match (lhs, rhs) {
        (Some(a), Some(b)) => Some(f(a, b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

impl<T: MinMax> MinMax for Option<T> {
    fn min_num(self, other: Option<T>) -> Option<T> {
        option_zip_some(self, other, MinMax::min_num)
    }

    fn max_num(self, other: Option<T>) -> Option<T> {
        option_zip_some(self, other, MinMax::max_num)
    }

    fn clamp_num(self, min: Option<T>, max: Option<T>) -> Option<T> {
        match self {
            Some(v) => match (min, max) {
                (Some(min_v), Some(max_v)) => Some(v.clamp_num(min_v, max_v)),
                (Some(min_v), None) => Some(v.max_num(min_v)),
                (None, Some(max_v)) => Some(v.min_num(max_v)),
                (None, None) => Some(v),
            },
            None => match (min, max) {
                (Some(min_v), None) => Some(min_v),
                (None, Some(max_v)) => Some(max_v),
                _ => None,
            },
        }
    }

    fn minimum(self, other: Option<T>) -> Option<T> {
        option_zip_some(self, other, MinMax::minimum)
    }

    fn maximum(self, other: Option<T>) -> Option<T> {
        option_zip_some(self, other, MinMax::maximum)
    }

    fn clamp(self, min: Option<T>, max: Option<T>) -> Option<T> {
        match self {
            Some(v) => match (min, max) {
                (Some(min_v), Some(max_v)) => Some(v.clamp(min_v, max_v)),
                (Some(min_v), None) => Some(v.maximum(min_v)),
                (None, Some(max_v)) => Some(v.minimum(max_v)),
                (None, None) => Some(v),
            },
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
                fn minimum(self, other: Self) -> Self {
                    if self < other { self } else { other }
                }

                fn maximum(self, other: Self) -> Self {
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
                fn minimum(self, other: Self) -> Self {
                    <$t>::min(self, other)
                }

                fn maximum(self, other: Self) -> Self {
                    <$t>::max(self, other)
                }
            }
        )*
    };
}

macro_rules! impl_min_max_float_ref {
    ($($t:ty),* $(,)?) => {
        $(

            impl MinMax for &$t {
                fn minimum(self, other: Self) -> Self {
                    if *self < *other { self } else { other }
                }

                fn maximum(self, other: Self) -> Self {
                    if *self > *other { self } else { other }
                }
            }
        )*
    };
}
impl_min_max_ord!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_min_max_ord!(&i8, &i16, &i32, &i64, &i128, &isize, &u8, &u16, &u32, &u64, &u128, &usize);
impl_min_max_float!(f32, f64);
impl_min_max_float_ref!(f32, f64);

#[macro_export]
macro_rules! min {
    ($first:expr $(,)?) => {{
        $first
    }};
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let mut acc = $first;
        $(
            acc = $crate::math::traits::cmp::MinMax::minimum(acc, $rest);
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
            acc = $crate::math::traits::cmp::MinMax::maximum(acc, $rest);
        )+
        acc
    }};
}
#[cfg(test)]
mod test{
    #[test]
    fn test_min_macro_0(){
        assert_eq!(min!(3.0f32, 2.0, 5.0, 1.0), 1.0);
    }
    #[test]
    fn test_max_macro_0(){
        assert_eq!(max!(3.0f32, 2.0, 5.0, 1.0), 5.0);
    }
}
