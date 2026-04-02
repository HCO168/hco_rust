use crate::math::traits::floatoid::IsNaN;

/// Trait for types that has a finite minimum.
pub trait MinValue {
    const MIN: Self;
}
/// Trait for types that has a finite maximum.
pub trait MaxValue {
    const MAX: Self;
}
/// Combination trait for types with both minimum and maximum constants.
/// Implement MIN/MAX constants for primitive numeric types.
macro_rules! impl_bounds_value {
    ($($t:ty),* $(,)?) => {
        $(
            impl MinValue for $t {
                const MIN: $t = <$t>::MIN;
            }
            impl MaxValue for $t {
                const MAX: $t = <$t>::MAX;
            }
        )*
    };
}
impl_bounds_value!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);



/// Min and Max operations that support partially ordered types.
pub trait MinMax: PartialOrd {
    /// Choose the min.
    /// - if exactly one side is NaN, return the other side
    /// - if both sides are NaN, return NaN
    fn min(self, other: Self) -> Self
    where Self: Sized{
        match self.partial_cmp(&other) {
            // if self is greater than other, return other
            Some(core::cmp::Ordering::Greater) => other,
            // otherwise return self
            _ => {
                if self.is_nan() { other }
                else { self }
            }
        }
    }
    /// Choose the max.
    /// - if exactly one side is NaN, return the other side
    /// - if both sides are NaN, return NaN
    fn max(self, other: Self) -> Self
    where Self: Sized{
        match self.partial_cmp(&other) {
            // if self is less than other, return other
            Some(core::cmp::Ordering::Less) => other,
            // otherwise return self
            _ => {
                if self.is_nan() { other }
                else { self }
            }
        }
    }
    /// Clamp value into range [min, max] using this trait's min/max semantics.
    /// Special cases when self is NaN to avoid output max when (NaN, Some, Some) is happening.
    fn clamp(self,min:Self,max:Self) -> Self
    where Self: Sized{
        // if the primary value is NaN, and sides are both NaN / both not NaN, return NaN
        // the NaN come from self
        if(self.is_nan()){
            // (nan,nan,whatever) return max(some or nan)
            // (nan,some,nan) return min(some)
            // (nan,some,some) return self(nan)
            return if (min.is_nan()) { max }
            else if (max.is_nan()) { min }
            else { self }
        }
        // deal with the special case of min > max
        // if min > max, swap them
        if(min<=max){
            if self < min { min }
            else if self > max { max }
            else { self }
        }else{
            if self < max { max }
            else if self > min { min }
            else { self }
        }
    }

    /// Apply min with an optional rhs; return None when rhs is None.
    fn min_opt(self,other:Option<Self>)-> Option<Self>
    where Self: Sized{
        match other {
            Some(value) => {Some(self.min(value))}
            None => {None}
        }
    }
    /// Apply max with an optional rhs; return None when rhs is None.
    fn max_opt(self,other:Option<Self>)-> Option<Self>
    where Self: Sized{
        match other {
            Some(value) => {Some(self.max(value))}
            None => {None}
        }
    }
    /// Clamp with optional bounds.
    /// Missing min/max bound means that side is not constrained.
    fn clamp_opt(self,min:Option<Self>,max:Option<Self>)-> Option<Self>
    where Self: Sized{
        match (min,max) {
            (Some(min),Some(max)) => Some(self.clamp(min,max)),
            (Some(min),None) => Some(self.max(min)),
            (None,Some(max)) => Some(self.min(max)),
            (None,None) => Some(self),
        }
    }
}
impl<T: MinMax> MinMax for Option<T> {
    fn min(self, other: Option<T>) -> Option<T>
    where
        Self: Sized,
    {
        match (self, other) {
            (Some(v1), Some(v2)) => Some(MinMax::min(v1, v2)),
            (Some(v1), None) => Some(v1),
            (None, Some(v2)) => Some(v2),
            (None, None) => None,
        }
    }
    fn max(self, other: Option<T>) -> Option<T>
    where
        Self: Sized,
    {
        match (self, other) {
            (Some(v1), Some(v2)) => Some(MinMax::max(v1, v2)),
            (Some(v1), None) => Some(v1),
            (None, Some(v2)) => Some(v2),
            (None, None) => None,
        }
    }
    fn clamp(self, min: Option<T>, max: Option<T>) -> Option<T>
    where
        Self: Sized,
    {
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


/// Implement MinMax for integer-like total-ordered primitives.
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
                    if(min<=max){
                        if self < min { min }
                        else if self > max { max }
                        else { self }
                    }else{
                        if self < max { max }
                        else if self > min { min }
                        else { self }
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
                        if self < min { min }
                        else if self > max { max }
                        else { self }
                    } else {
                        if self < max { max }
                        else if self > min { min }
                        else { self }
                    }
                }
            }
        )*
    };
}
impl_min_max_ord!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
/// Implement MinMax for basic types
macro_rules! impl_min_max_float {
    ($($t:ty),*$(,)?) => {
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
impl_min_max_float!(f32, f64);


/// Provide min/max macros for multiple arguments.
#[cfg(test)]
macro_rules! min {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        let mut min = $first;
        $(
            min = MinMax::min(min, $rest);
        )*
        min
    }};
}
#[cfg(test)]
macro_rules! max {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        let mut max = $first;
        $(
            max = MinMax::max(max, $rest);
        )*
        max
    }};
}


#[cfg(test)]
mod test{
    use super::*;
    fn assert_f32_eq_or_nan(left: f32, right: f32) {
        if left.is_nan() && right.is_nan() {
            return;
        }
        assert_eq!(left, right);
    }

    fn assert_opt_f32_eq_or_nan(left: Option<f32>, right: Option<f32>) {
        match (left, right) {
            (None, None) => {}
            (Some(a), Some(b)) => assert_f32_eq_or_nan(a, b),
            _ => panic!("option mismatch"),
        }
    }

    #[test]
    fn test_min_macro(){
        assert_eq!(
            min!(4i32,3,2,1,0,-1,-2,-3,-4),
            -4
        )
    }
    #[test]
    fn test_max_macro(){
        assert_eq!(
            max!(4,3,2,1,0,-1,-2,-3,-4),
            4
        )
    }
    #[test]
    fn test_min_max_ord_values() {
        assert_eq!(MinMax::min(5i32, 2), 2);
        assert_eq!(MinMax::max(5i32, 2), 5);
        assert_eq!(MinMax::clamp(5i32, 1, 3), 3);
        assert_eq!(MinMax::clamp(2i32, 5, 1), 2); // reversed bounds are handled
    }

    #[test]
    fn test_min_max_float_with_nan() {
        assert_f32_eq_or_nan(MinMax::min(1.0f32, f32::NAN), 1.0);
        assert_f32_eq_or_nan(MinMax::min(f32::NAN, 1.0), 1.0);
        assert!(MinMax::min(f32::NAN, f32::NAN).is_nan());

        assert_f32_eq_or_nan(MinMax::max(1.0f32, f32::NAN), 1.0);
        assert_f32_eq_or_nan(MinMax::max(f32::NAN, 1.0), 1.0);
        assert!(MinMax::max(f32::NAN, f32::NAN).is_nan());
    }

    #[test]
    fn test_clamp_float_with_nan_and_bounds() {
        assert!(MinMax::clamp(f32::NAN, 0.0, 1.0).is_nan());
        assert_f32_eq_or_nan(MinMax::clamp(f32::NAN, 0.0, f32::NAN), 0.0);
        assert_f32_eq_or_nan(MinMax::clamp(f32::NAN, f32::NAN, 1.0), 1.0);
    }

    #[test]
    fn test_min_opt_max_opt_clamp_opt_for_value() {
        assert_opt_f32_eq_or_nan(3.0f32.min_opt(Some(1.0)), Some(1.0));
        assert_opt_f32_eq_or_nan(3.0f32.min_opt(None), None);

        assert_opt_f32_eq_or_nan(3.0f32.max_opt(Some(5.0)), Some(5.0));
        assert_opt_f32_eq_or_nan(3.0f32.max_opt(None), None);

        assert_opt_f32_eq_or_nan(3.0f32.clamp_opt(Some(1.0), Some(2.0)), Some(2.0));
        assert_opt_f32_eq_or_nan(3.0f32.clamp_opt(Some(1.0), None), Some(3.0));
        assert_opt_f32_eq_or_nan(3.0f32.clamp_opt(None, Some(2.0)), Some(2.0));
        assert_opt_f32_eq_or_nan(3.0f32.clamp_opt(None, None), Some(3.0));
    }

    #[test]
    fn test_option_min_max_clamp_with_option_rhs() {
        assert_opt_f32_eq_or_nan(Some(3.0f32).min(Some(1.0)), Some(1.0));
        assert_opt_f32_eq_or_nan(Some(3.0f32).min(None), Some(3.0));
        assert_opt_f32_eq_or_nan(None::<f32>.min(Some(1.0)), Some(1.0));
        assert_opt_f32_eq_or_nan(None::<f32>.min(None), None);

        assert_opt_f32_eq_or_nan(Some(3.0f32).max(Some(5.0)), Some(5.0));
        assert_opt_f32_eq_or_nan(Some(3.0f32).max(None), Some(3.0));
        assert_opt_f32_eq_or_nan(None::<f32>.max(Some(5.0)), Some(5.0));
        assert_opt_f32_eq_or_nan(None::<f32>.max(None), None);

        assert_opt_f32_eq_or_nan(Some(3.0f32).clamp(Some(1.0), Some(2.0)), Some(2.0));
        assert_opt_f32_eq_or_nan(Some(3.0f32).clamp(Some(1.0), None), Some(3.0));
        assert_opt_f32_eq_or_nan(Some(3.0f32).clamp(None, Some(2.0)), Some(2.0));
        assert_opt_f32_eq_or_nan(None::<f32>.clamp(Some(1.0), None), Some(1.0));
        assert_opt_f32_eq_or_nan(None::<f32>.clamp(None, Some(2.0)), Some(2.0));
        assert_opt_f32_eq_or_nan(None::<f32>.clamp(Some(1.0), Some(2.0)), None);
    }

    #[test]
    fn test_option_with_nan_values() {
        assert_opt_f32_eq_or_nan(Some(f32::NAN).min(Some(1.0)), Some(1.0));
        assert_opt_f32_eq_or_nan(Some(1.0).min(Some(f32::NAN)), Some(1.0));
        assert_opt_f32_eq_or_nan(Some(f32::NAN).max(Some(1.0)), Some(1.0));
        assert_opt_f32_eq_or_nan(Some(1.0).max(Some(f32::NAN)), Some(1.0));
    }
    #[test]
    fn test_min_max_with_ref(){
        assert_eq!(MinMax::clamp(&3f32,&1f32,&2f32),&2f32);
    }
    pub struct TestStruct<T>{
        a:T,
    }
    impl<T:MinMax> TestStruct<T>{
        fn max2(self, other:Self) -> T{
            self.a.max(other.a)
        }
    }
}
