pub const fn gcd_stein(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b.abs();
    }
    if b == 0 {
        return a.abs();
    }

    let shift = (a | b).trailing_zeros();

    a = a.abs();
    b = b.abs();

    a >>= a.trailing_zeros();
    while b != 0 {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
    }
    a << shift
}
#[inline(always)]
pub fn get_bit<T>(value: T, index: usize) -> bool
where
    T: Copy + std::ops::Shr<usize, Output = T> + std::ops::BitAnd<Output = T> + PartialEq + From<u8>,
{
    ((value >> index) & T::from(1)) == T::from(1)
}
