pub trait AdditiveIdentity {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}
pub trait MultiplicativeIdentity {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}
pub trait AdditiveInverse<T> {
    fn neg(&self) -> T;
}
