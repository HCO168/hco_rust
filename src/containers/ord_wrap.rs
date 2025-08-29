use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq,PartialOrd)]
pub struct Ordered<T>(T)
where T:PartialOrd;
impl<T> Ordered<T>where T:PartialOrd{
    fn from(v: T) -> Self {
        Ordered::new(v).expect("value must satisfy v == v")
    }
    pub fn new(v:T)->Option<Ordered<T>>{
        if(v==v){
            Some(Ordered(v))
        }else{
            None
        }
    }
    pub fn unbox(self) -> T {
        self.0
    }
    pub fn set(&mut self, v: T) -> bool {
        if (v==v) {
            self.0 = v;
            true
        } else {
            false
        }
    }
}

impl<T: PartialOrd> Eq for Ordered<T> {
}

impl<T:PartialOrd> Ord for Ordered<T>{
    fn cmp(&self,other:&Ordered<T>)->Ordering{
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl<T:Default + PartialOrd> Default for Ordered<T> {
    fn default() -> Ordered<T> {
        Ordered(T::default())
    }
}
impl<T:Display+PartialOrd> Display for Ordered<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<T:PartialOrd> Deref for Ordered<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: PartialOrd> AsRef<T> for Ordered<T> {
    fn as_ref(&self) -> &T { &self.0 }
}

#[cfg(test)]
pub mod test{
    pub use super::*;
    #[test]
    pub fn test() {
        let a=Ordered::from(1.7);
        let b=Ordered::from(1.7);
        assert_eq!(a,b);
    }
    #[test]
    #[should_panic(expected="value must satisfy v == v")]
    pub fn test2() {
        let a=Ordered::from(f32::NAN);
    }
}