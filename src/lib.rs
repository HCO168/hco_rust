#![allow(unused_parens)]
#![allow(dead_code)]
//re-exports

#[cfg(feature = "math")]
mod math;
#[cfg(feature = "data")]
mod data;
#[cfg(feature = "ntr_lang")]
mod ntr_lang;
mod tool;

#[cfg(test)]
mod tests {
    use std::ops::Neg;

    #[test]
    fn it_works() {
    }
}
