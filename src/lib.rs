#![allow(unused_parens)]
#![allow(dead_code)]


mod math;
#[cfg(feature = "tool")]
mod tool;
#[cfg(feature = "container")]
mod containers;
mod containers;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
