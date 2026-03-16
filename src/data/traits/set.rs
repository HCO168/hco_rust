trait Contains<T>{
    fn contains(&self, x:T) -> bool;
}
trait Insert<T>{
    fn insert(&mut self, x:T);
}
trait Remove<T>: Contains<T>{
    fn remove(&mut self, x:T)->bool;
}
