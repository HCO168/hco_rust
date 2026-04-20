#![allow(unused_parens)]
#![allow(dead_code)]
#![cfg_attr(feature = "specialization", feature(min_specialization))]
//re-exports

pub mod math;
pub mod datas;
pub mod ntr_lang;
pub mod tool;
mod traits;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
