use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq,PartialOrd)]
pub struct Ordered<T>(T)
where T:PartialOrd;
impl<T> Ordered<T>where T:PartialOrd{
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