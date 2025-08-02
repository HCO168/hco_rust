
pub trait NAN {
    fn is_nan(&self) -> bool;
}
impl<T: PartialEq> NAN for T {
    fn is_nan(&self) -> bool {
        self != self
    }
}