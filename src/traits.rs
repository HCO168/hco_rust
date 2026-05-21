#[cfg(test)]
pub mod test{
    pub trait Test{}
    impl<T:crate::math::traits::MulId> Test for T{}
}