pub use crate::math::traits::*;
pub use crate::datas::traits::*;
#[cfg(test)]
pub mod test{
    pub trait Test{}
    impl<T:crate::traits::MulId> Test for T{}
}