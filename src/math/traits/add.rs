pub trait AddId {
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