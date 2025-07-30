use crate::container::array::Array;

pub struct ContinuedSets<T>
where T:PartialOrd,{
    points: Vec<T>,
    exclude_points: Vec<T>,
    intervals: Vec<(T,T)>
}
impl<T: std::cmp::PartialOrd> ContinuedSets<T>{
    fn add_point(&mut self, point: T){
        
    }
}