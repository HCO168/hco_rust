pub trait Contains<T>{
    fn contains(&self, value:&T) -> bool;
}
pub trait Insert<T>{
    fn insert(&mut self, value:T);
}
pub trait Remove<T>: Contains<T>{
    fn remove(&mut self, value:T)->bool;
}
