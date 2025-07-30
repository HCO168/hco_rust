use std::ops::Range;

pub struct Interval<T>where T:PartialOrd{
    lf:T,
    rt:T,
    flag:IntervalFlag
}
impl<T> Interval<T> where T:PartialOrd{
    /**
    should not put NaN into args
    */
    pub fn new(a:T,b:T,left_open:bool,right_open:bool)->Interval<T>{
        //check NaN when debug
        debug_assert!(
            a==a&&b==b,
            "Interval endpoints cannot be NaN"
        );
        if(b>a){
            Interval{ lf: a, rt: b,flag:IntervalFlag::new(left_open, right_open)}
        }else{
            Interval{ rt: b, lf: a,flag:IntervalFlag::new(right_open, left_open)}
        }
    }
    pub fn contain(&self,value:&T)->bool{(
        if(self.left_open()){*value>self.lf }else{*value>=self.lf }//check left
        &&
        if(self.right_open()){*value<self.rt }else{*value<=self.rt } //check right
    )}
    pub fn left_open(&self) -> bool{self.flag.left_open()}
    pub fn right_open(&self) -> bool{self.flag.right_open()}
    pub fn is_overlap(&self,other:&Interval<T>)->bool{(
        if(self.left_open()||other.right_open()){
            other.rt>self.lf
        }else{
            other.rt>=self.lf
        }&&if(self.right_open()||other.left_open()){
            self.rt>other.lf
        }else{
            self.rt>=other.rt
        }
    )}
}
pub struct IntervalFlag{
    state:u8
}
impl IntervalFlag{
    pub fn new(left_open:bool, right_open:bool)->IntervalFlag{
        IntervalFlag{state: (left_open as u8)<<1|right_open as u8 }
    }
    pub fn left_open(&self)->bool{self.state&0b00000010!=0}
    pub fn right_open(&self)->bool{self.state&0b00000001!=0}
}