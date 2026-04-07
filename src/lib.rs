#![allow(unused_parens)]
#![allow(dead_code)]
#![feature(min_specialization)]
//re-exports

#[cfg(feature = "math")]
pub mod math;
#[cfg(feature = "data")]
pub mod datas;
#[cfg(feature = "ntr_lang")]
pub mod ntr_lang;
pub mod tool;

#[cfg(test)]
mod tests {
    use std::ops::Neg;

    #[test]
    fn it_works() {

    }
}
