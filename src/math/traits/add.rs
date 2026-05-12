pub trait AddId: Sized {
    const ZERO: Self;
}

macro_rules! impl_add_id_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl AddId for $t {
                const ZERO: Self = 0;
            }
        )*
    };
}

macro_rules! impl_add_id_float {
    ($($t:ty),* $(,)?) => {
        $(
            impl AddId for $t {
                const ZERO: Self = 0.0;
            }
        )*
    };
}
impl_add_id_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_add_id_float!(f32, f64);

pub trait IsAddId:AddId+PartialEq {
    fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }

    fn not_zero(&self) -> bool {
        !self.is_zero()
    }
}

macro_rules! impl_is_add_id {
    ($($t:ty),* $(,)?) => {
        $(
            impl IsAddId for $t {
                fn is_zero(&self) -> bool {
                    self == &Self::ZERO
                }
            }
        )*
    };
}
impl_is_add_id!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,f32, f64);
