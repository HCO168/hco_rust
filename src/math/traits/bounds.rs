use super::multiplicative::IsNaN;

/// Trait for types that has a minimum.
pub trait MinValue {
    const MIN: Self;
}
/// Trait for types that has a maximum.
pub trait MaxValue {
    const MAX: Self;
}
/// Combination trait for types with both minimum and maximum constants.
pub trait Bounds: MinValue + MaxValue {}
impl<T: MinValue + MaxValue> Bounds for T {}
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


/// Min operations that support partially ordered types.
pub trait PartialMin: PartialOrd{
    /// Choose the min, but if any is NaN, return none
    fn partial_min(self, other: Self) -> Option<Self>
    where Self: Sized{
        match self.partial_cmp(&other) {
            // if self is greater than other, return other
            Some(core::cmp::Ordering::Greater) => Some(other),
            // otherwise return self
            Some(_) => Some(self),
            // if both are NaN, return None
            None => None,
        }
    }
    /// Choose the min, but only return none while both is none
    /// If there is any one NaN, the other will be returned
    fn partial_min_select(self, other: Self) -> Option<Self>
    where Self: Sized + IsNaN{
        if self.is_nan() {
            // if both are NaN, return None
            if other.is_nan() { None }
            // if self is NaN, return other
            else { Some(other) }
        }
        // if self in not NaN
        else{
            // if other is NaN, return self
            if other.is_nan() { Some(self) }
            // if both are not NaN, return partial min
            else {self.partial_min(other)}
        }
    }
}


/// Max operations that support partially ordered types.
pub trait PartialMax: PartialOrd{
    /// Choose the min, but if any is NaN, return none
    fn partial_min(self, other: Self) -> Option<Self>
    where Self: Sized{
        match self.partial_cmp(&other) {
            // if self is greater than other, return other
            Some(core::cmp::Ordering::Greater) => Some(other),
            // otherwise return self
            Some(_) => Some(self),
            // if both are NaN, return None
            None => None,
        }
    }
    /// Choose the min, but only return none while both is none
    /// If there is any one NaN, the other will be returned
    fn partial_min_select(self, other: Self) -> Option<Self>
    where Self: Sized + IsNaN{
        if self.is_nan() {
            // if both are NaN, return None
            if other.is_nan() { None }
            // if self is NaN, return other
            else { Some(other) }
        }
        // if self in not NaN
        else{
            // if other is NaN, return self
            if other.is_nan() { Some(self) }
            // if both are not NaN, return partial min
            else {self.partial_min(other)}
        }
    }
}