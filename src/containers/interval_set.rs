use std::collections::BTreeSet as OrderedSet;
use std::collections::BTreeMap as OrderedMap;
use std::fmt::Formatter;
use std::fmt::Display;
use crate::containers::interval_set::PointStatus::{InExclude, InInterval, InInclude, Outside};
use crate::containers::ord_wrap::Ordered;
use crate::math::interval::*;
use crate::containers::ordered::*;

pub struct IntervalSet<T>
where T:Ord,{
    points: OrderedSet<T>,//in intervals means exclude. vice versa
    intervals: OrderedMap<T,T>,//lf,rt,closed Range
}
enum PointStatus {
    InInclude,
    InExclude,
    InInterval,
    Outside,
}
impl<T: Ord> IntervalSet<T>{
    pub fn contains_point(&self, value: &T) -> bool {
        match self.classify_point(value){
            InInclude => true,
            InExclude => false,
            InInterval => true,
            Outside => false,
        }
    }
    pub fn add_point(&mut self, point: T){
        let status=self.classify_point(&point);
        match status {
            Outside => {
                self.points.insert(point);
            }
            InExclude => {
                self.points.remove(&point);
            }
            _=>{}
        }
    }

    pub fn remove_point(&mut self, point: T){
        let status=self.classify_point(&point);
        match status {
            InInclude => {
                self.points.remove(&point);
            }
            InInterval => {
                self.points.insert(point);
            }
            _=>{}
        }
    }
    #[inline]
    fn intervals_contain(&self, point: &T)->bool{
        match self.intervals.left_of(point) {
            None => false,
            Some((_, right)) => point<=right,
        }
    }
    fn classify_point(&self, point: &T) -> PointStatus{
        //check if in Interval
        if(self.intervals_contain(point)){
            if(self.points.contains(point)){
                InExclude
            }else{
                InInterval
            }
        }else{
            if(self.points.contains(point)){
                InInclude
            }else{
                Outside
            }
        }
    }
}

impl<T: Ord+Clone> IntervalSet<T>{
    pub fn add_interval(&mut self,interval: Interval<T>){
        //destruct the interval
        let (itv_lf,itv_rt,flag)=interval.take_away();

        //store the status of the points and use later
        let left_status=self.classify_point(&itv_lf);
        let right_status=self.classify_point(&itv_rt);


        //clear all points in [itv_lf,ltv_rt] because the new interval are covering them
        // no matter exclude or include
        //we do not need to care about the points in [new_left,itv_lf) and (itv_rt,new right]
        //because they must mean excluding points no matter originally or when finished
        let keys_to_delete=self.points.collect_keys(&itv_lf..=&itv_rt);
        for key in keys_to_delete {
            self.points.remove(&key);
        }
        
        //if any closed interval overlap with [the new interval as closed interval]
        //remove that interval and store the left (or right) as the new left (or right)
        let new_left =match(self.intervals.left_of(&itv_lf)){
            Some((left,right)) if(&itv_lf<=right)=> {
                match self.intervals.remove_entry(&left.clone()) {
                    None => {itv_lf.clone()}
                    Some((left,_)) => {left}
                }
            }
            _ => {itv_lf.clone()}
        };
        let new_right =match(self.intervals.left_of(&itv_rt)){
            Some((left,right)) if(&itv_rt<=right)=> {
                match self.intervals.remove_entry(&left.clone()) {
                    None => {itv_rt.clone()}
                    Some((_,right)) => {right}
                }
            }
            _ => {itv_rt.clone()}
        };
        //remove all the intervals in between our interval because they are covered
        let keys_to_delete=self.intervals.collect_keys(&itv_lf..=&itv_rt);
        for key in keys_to_delete {
            self.intervals.remove(&key);
        }
        
        //add the new interval so all the range are covered
        self.intervals.insert(new_left.clone(), new_right.clone());

        //consider the original interval left status
        match left_status {
            InExclude => {
                //if it is in exclude and our interval is left closed, 
                //remove the point so that it is not exclude
                if(!flag.left_open()){
                    self.points.remove(&itv_lf);
                }
            }
            Outside => {
                //if the original point is outside and out interval is left open
                //add the point to exclude it
                if(flag.left_open()){
                    self.points.insert(itv_lf);
                }
            }
            //we do nothing if the point is included originally
            _=>{}
            
        }
        //same as the right
        match right_status {
            InExclude => {
                if(!flag.right_open()){
                    self.points.remove(&itv_rt);
                }
            }
            Outside => {
                if(flag.right_open()){
                    self.points.insert(itv_rt);
                }
            }
            _=>{}
        }
    }
}
impl<T:Ord+ToString+Default> Display for IntervalSet<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut points_iter=self.points.iter();
        let mut interval_iter=self.intervals.iter();
        let mut current_point=points_iter.next();
        let mut current_interval =interval_iter.next();
        let default = T::default();
        let mut helper=SetOutputHelper::new(&default);
        while(current_point.is_some()||current_interval.is_some()){
            match current_point{
                //when there is some points left
                Some(point)=>{
                    match current_interval {
                        //When there is intervals left
                        Some(interval)=>{
                            //when the point overlap with current interval left
                            if(interval.0)<=point{
                                //add the interval with whether left is open, update the interval
                                if(interval.0==point){
                                    helper.add_interval(interval.0,true,interval.1);
                                }else{
                                    helper.add_interval(interval.0,false,interval.1);
                                    helper.add_point(point)
                                }
                                current_interval=interval_iter.next();
                            }else{
                                //when the point are not overlapping with the current interval,
                                //normally add it to the helper and do not update the interval
                                helper.add_point(point)
                            }
                        }
                        //when no interval left, just normally add point
                        //the helper will take care of the right point
                        None=>{
                            helper.add_point(point)
                        }
                    }
                    current_point=points_iter.next();
                }
                //when there is no points left
                None=>{
                    //add all intervals to the helper
                    while(current_interval.is_some()){
                        match current_interval {
                            Some(interval)=>{
                                helper.add_interval(interval.0,false,interval.1);
                                helper.finish_right(false);
                                current_interval=interval_iter.next();
                            }
                            None=>{
                                break;
                            }
                        }
                    }
                }
            }
        }
        f.write_str(&helper.finish())
    }

}
struct SetOutputHelper<'a,T>{
    output:String,
    in_interval:bool,
    current_interval_right:&'a T
}
impl<'a,T:Ord+ToString> SetOutputHelper<'a,T>{
    fn new(initial: &'a T)->Self{
        return SetOutputHelper{
            output:String::new(),
            in_interval:false,
            current_interval_right:initial
        };
    }
    fn add_interval(&mut self, left:&T, left_open:bool,point:&'a T){
        //form something like [-5,
        //and store that we are in a interval
        self.in_interval = true;
        self.output.push(if(left_open){'('}else{'['});
        self.output.push_str(left.to_string().as_str());
        self.output.push(',');
        self.current_interval_right=point;
    }
    fn add_point(&mut self,point:&T){
        //treat points in interval as excluded and outside interval as included
        if(self.in_interval){
            //When the point is possible in the interval
            if(point<self.current_interval_right){
                //if the point is in the interval, make something like [5,7)(7,10]
                self.output.push_str(&point.to_string());
                self.output.push_str("),(");
                self.output.push_str(&point.to_string());
                self.output.push(',');
                return;
            }else if(point==self.current_interval_right){
                //if the point is at the interval right point, close the interval with open bracket ')'
                self.finish_right(true);
                return;
            }else{
                //if the point is larger than the interval right point,
                // close the interval with open bracket ')'
                self.finish_right(false);
                //and continue to add the point as an exclude point
            }
        }
        //form something like ≠7.5,
        self.output.push('≠');
        self.output.push_str(&point.to_string());
        self.output.push(',');
    }
    fn finish_right(&mut self,open:bool){
        //finish the interval right when it is actually need to finish
        //generate something like 114),
        if(self.in_interval){
            self.output.push_str(self.current_interval_right.to_string().as_str());
            self.output.push(if(open){')'}else{']'});
            self.output.push(',');
            self.in_interval = false;
        }
    }
    fn finish(mut self) ->String{
        if(self.in_interval){
            self.finish_right(false);
        }
        self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap as OrderedMap, BTreeSet as OrderedSet};

    // ---- 小工具 ----------------------------------------------------------------
    fn cs_i32() -> IntervalSet<i32> {
        IntervalSet {
            points: OrderedSet::new(),
            intervals: OrderedMap::new(),
        }
    }
    fn cs_f64() -> IntervalSet<Ordered<f64>> {
        IntervalSet {
            points: OrderedSet::new(),
            intervals: OrderedMap::new(),
        }
    }
    macro_rules! ivl {
        ($l:expr, $r:expr, $lo:expr, $ro:expr) => {
            Interval::new($l, $r, $lo, $ro)
        };
    }

    // ---- 1. 只有一点 ------------------------------------------------------------
    #[test]
    fn add_single_point() {
        let mut s = cs_i32();
        s.add_point(5);
        assert_eq!(format!("{s}"), "≠5,");
    }

    // ---- 2. 只有闭区间 ----------------------------------------------------------
    #[test]
    fn add_single_closed_interval() {
        let mut s = cs_i32();
        s.add_interval(ivl!(1, 10, false, false));   // [1,10]
        assert_eq!(format!("{s}"), "[1,10],");
    }

    // ---- 3. 区间内排除一点（将区间劈成两段） ------------------------------------
    #[test]
    fn exclude_point_inside_interval() {
        let mut s = cs_i32();
        s.add_interval(ivl!(1, 10, false, false));   // [1,10]
        s.remove_point(5);                           // 排除 5
        assert_eq!(format!("{s}"), "[1,5),(5,10],");
    }

    // ---- 4. 区间 + 区间外一点 ---------------------------------------------------
    #[test]
    fn mix_point_and_interval() {
        let mut s = cs_i32();
        s.add_interval(ivl!(1, 10, false, false));
        s.remove_point(5);   // exclusion
        s.add_point(0);      // inclusion
        assert_eq!(format!("{s}"), "≠0,[1,5),(5,10],");
    }

    // ---- 5. 相接区间合并 --------------------------------------------------------
    #[test]
    fn merge_touching_intervals() {
        let mut s = cs_i32();
        s.add_interval(ivl!(1, 10, false, false));   // [1,10]
        s.add_interval(ivl!(10, 20, true, false));   // (10,20] ——左端开放但与前区间相接
        assert_eq!(format!("{s}"), "[1,20],");       // 应合并为 [1,20]
    }

    // ---- 6. 通过排除右端点实现右开区间 -----------------------------------------
    #[test]
    fn open_right_bound_by_excluding_endpoint() {
        let mut s = cs_i32();
        s.add_interval(ivl!(1, 20, false, false));   // 先加闭区间
        s.remove_point(20);                          // 排除右端点
        assert_eq!(format!("{s}"), "[1,20),");       // 变成右开
    }

    // ---- 7. 加入不相交区间 & 区间外点 ------------------------------------------
    #[test]
    fn add_disjoint_interval() {
        let mut s = cs_i32();
        s.add_interval(ivl!(1, 20, false, true));    // [1,20)
        s.add_interval(ivl!(30, 40, false, false));  // 新的闭区间
        s.add_point(0);                              // 区间外 inclusion
        assert_eq!(format!("{s}"), "≠0,[1,20),[30,40],");
    }

    // ---- 8. 重叠区间合并 -------------------------------------------------------
    #[test]
    fn merge_overlapping_intervals() {
        let mut s = cs_i32();
        s.add_interval(ivl!(30, 40, false, false));
        s.add_interval(ivl!(25, 35, false, false));  // 与前者交叠
        assert_eq!(format!("{s}"), "[25,40],");
    }

    // ---- 9. 浮点区间 + 排除中点 -------------------------------------------------
    #[test]
    fn float_interval_and_exclude_mid_point() {
        let mut s = cs_f64();
        s.add_interval(ivl!(
            Ordered::new(0.0).unwrap(),
            Ordered::new(1.0).unwrap(),
            false,
            false
        ));
        s.remove_point(Ordered::new(0.5).unwrap());  // 排除 0.5
        assert_eq!(format!("{s}"), "[0,0.5),(0.5,1],");
    }

    // ---- 10. 两端开放区间（通过排除端点实现） -----------------------------------
    #[test]
    fn open_interval_both_sides() {
        let mut s = cs_f64();
        // 先整体加入 (0,1) ——实现开放需排除端点
        s.add_interval(ivl!(
            Ordered::new(0.0).unwrap(),
            Ordered::new(1.0).unwrap(),
            true,
            true
        ));
        s.remove_point(Ordered::new(0.0).unwrap());
        s.remove_point(Ordered::new(1.0).unwrap());
        assert_eq!(format!("{s}"), "(0,1),");
    }
}
