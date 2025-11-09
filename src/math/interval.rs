use std::fmt::{Display, Formatter};
use std::ops::{Bound, Range, RangeBounds, RangeInclusive};

pub struct Interval<T>where T:Ord{
    range: ClosedRange<T>,
    flag:IntervalFlag
}
impl<T> Interval<T> where T:Ord{
    pub fn new(left:T,right:T,left_open:bool,right_open:bool)->Interval<T>{
        if(right>left){
            return Interval::new_valid(left,right,left_open,right_open);
        }else{
            return Interval::new_valid(right,left,right_open,left_open);
        }
    }
    fn new_valid(a:T,b:T,left_open:bool,right_open:bool)->Interval<T>{
        Interval{ range:ClosedRange{lf:a,rt:b},flag:IntervalFlag::new(left_open, right_open)}
    }
    pub fn from_closed(range: ClosedRange<T>) -> Self {
        Interval { range, flag: IntervalFlag::new(false, false) }
    }
    
    
    
    pub fn contain(&self,value:&T)->bool{(
        if(self.left_open()){value>self.left()}else{value>=self.left()}//check left
        &&
        if(self.right_open()){value<self.right() }else{value<=self.right()} //check right
    )}
    pub const fn left(&self)->&T{&self.range.lf}
    pub const fn right(&self)->&T{&self.range.rt}
    pub const fn left_open(&self) -> bool{self.flag.left_open()}
    pub const fn right_open(&self) -> bool{self.flag.right_open()}
    pub fn is_overlap(&self,other:&Interval<T>)->bool{(
        if(self.left_open()||other.right_open()){
            other.right()>self.left()
        }else{
            other.right()>=self.left()
        }&&if(self.right_open()||other.left_open()){
            self.right()>other.left()
        }else{
            self.right()>=other.left()
        }
    )}

    pub fn take_away(self)->(T,T,IntervalFlag){
        (self.range.lf,self.range.rt,self.flag)
    }
}

impl<T:Ord> RangeBounds<T> for Interval<T> {
    fn start_bound(&self) -> Bound<&T> {
        if(self.left_open()){
            Bound::Excluded(self.left())
        }else{
            Bound::Included(self.left())
        }
    }
    fn end_bound(&self) -> Bound<&T> {
        if(self.right_open()){
            Bound::Excluded(self.right())
        }else{
            Bound::Included(self.right())
        }
    }
}
impl<T:Display + std::cmp::Ord> Display for Interval<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{},{}{}", if(self.left_open()){"("}else{"["}, self.left().to_string(),
        self.right().to_string(),if(self.right_open()){"("}else{"["})
    }
}




pub struct IntervalFlag{
    state:u8
}
impl IntervalFlag{
    pub fn new(left_open:bool, right_open:bool)->IntervalFlag{
        IntervalFlag{state: (left_open as u8)<<1|right_open as u8 }
    }
    pub const fn left_open(&self)->bool{self.state&0b00000010!=0}
    pub const fn right_open(&self)->bool{self.state&0b00000001!=0}
}



#[derive(Debug, Copy, Clone,PartialEq,Eq)]
pub struct ClosedRange<T>
where T: Ord {
    lf: T,
    rt: T,
}
impl<T: Ord> ClosedRange<T> {
    pub fn new(lf: T, rt: T)->ClosedRange<T>{
        ClosedRange{lf, rt }
    }
    
    pub fn contains(&self, value: &T) -> bool {
        self.lf <= *value && *value <= self.rt
    }

    pub fn is_overlap(&self, other: &ClosedRange<T>) -> bool {
        other.rt >= self.lf && self.rt >= other.lf
    }
    
    pub fn left(&self)->&T{&self.lf}
    pub fn right(&self)->&T{&self.rt}
    pub fn take_away(self)->(T,T){
        (self.lf,self.rt)
    }
    
    
    pub fn into_range(self)->Range<T>{
        self.lf..self.rt
    }
    pub fn to_range(&self) -> RangeInclusive<T>
    where T: Clone {
        self.lf.clone()..=self.rt.clone()
    }
}
