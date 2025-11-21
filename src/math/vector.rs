use std::ops::{Add, AddAssign};

pub struct Vector<T,const N:usize> {
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
impl<T:Add<Output=T>+Default+Copy,const N:usize> Add for Vector<T,N>{
    type Output = Vector<T,N>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut array=[T::default(); N];
        for i in 0..N{
            array[i]=self.data[i]+rhs.data[i];
        }
        Vector::new(array)
    }
}
impl<T:AddAssign+Default+Copy,const N:usize> AddAssign for Vector<T,N>{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N{
            self.data[i]+=rhs.data[i];
        }
    }
}
