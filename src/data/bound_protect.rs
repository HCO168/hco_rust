use std::ops::Index;

///Index Protector for array when out of bound
pub enum ProtectStrategy<V> {
    Closest,
    Loop,
    Default(V)
}
pub trait BoundProtect<I,V> {
    fn get(&self, index: I) -> Option<&V>;
    fn get_safe(&self, index: I) ->&V;
    fn get_mut(&mut self, index: I) -> Option<&mut V>;
    fn get_mut_safe(&mut self, index: I) ->&mut V;
    fn set(&mut self, index: I, value: V) -> Option<V>;
    fn set_safe(&mut self, index: I, value: V) -> V;
}

pub struct BoundProtector<'a,V,const D: usize> {
    strategy: ProtectStrategy<V>,
    array: &'a mut [usize; D],

}