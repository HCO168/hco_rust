pub trait ConstAddId{
    const ZERO: Self;
}
pub trait AddId: Sized {
    fn zero() -> Self;
}

macro_rules! impl_const_add_id_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl ConstAddId for $t {
                const ZERO: Self = 0;
            }
        )*
    };
}

macro_rules! impl_const_add_id_float {
    ($($t:ty),* $(,)?) => {
        $(
            impl ConstAddId for $t {
                const ZERO: Self = 0.0;
            }
        )*
    };
}
impl_const_add_id_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_const_add_id_float!(f32, f64);

// Non-specialization mode: one ConstAddId impl is enough to get AddId.
#[cfg(not(feature = "specialization"))]
impl<T:ConstAddId> AddId for T {
    fn zero() -> Self{
        Self::ZERO
    }
}

// Specialization mode: keep the same autocomplete path, but allow faster AddId impls.
#[cfg(feature = "specialization")]
impl<T:ConstAddId> AddId for T {
    default fn zero() -> Self{
        Self::ZERO
    }
}

pub trait IsAddId:AddId+PartialEq {
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    fn not_zero(&self) -> bool {
        !self.is_zero()
    }
}

// Keep IsAddId explicit. In stable mode this lets types optimize is_zero; in
// min_specialization mode it avoids specializing on generic bounds.
macro_rules! impl_is_add_id {
    ($($t:ty),* $(,)?) => {
        $(
            impl IsAddId for $t {
                fn is_zero(&self) -> bool {
                    self == &<Self as ConstAddId>::ZERO
                }
            }
        )*
    };
}
impl_is_add_id!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,f32, f64);
