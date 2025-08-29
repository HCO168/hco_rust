struct Vector<T,const N:usize> {
    data: [T; N],
}
impl<T,const N:usize> Vector<T,N> {
    pub fn new(data: [T; N]) -> Self{
        Vector{data}
    }
    pub fn dim(&self)->usize{
        N
    }
}