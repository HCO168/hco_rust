use std::cmp::Ordering;
use std::collections::BTreeSet as OrderedSet;
use std::collections::BTreeMap as OrderedMap;
use crate::containers::continued_sets::PointStatus::{InExclude, InInterval, InInclude, Outside};
use crate::containers::continued_sets::Status::{Exclude, Include};
use crate::containers::ord_wrap::Ordered;
use crate::math::interval::*;
use crate::containers::ordered::*;

pub struct ContinuedSets<T>
where T:Ord,{
    points: OrderedMap<T,Status>,
    intervals: OrderedMap<T,T>,//lf,rt,closed Range
}
enum Status{
    Include,
    Exclude,
}
impl<T: Ord> ContinuedSets<T>{
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
                self.points.insert(point,Include);
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
                self.points.insert(point,Exclude);
            }
            _=>{}
        }
    }
    fn classify_point(&self, point: &T) -> PointStatus{
        let left_option = self.intervals.left_of(point);
        //check if in Interval
        if(match left_option {
            None => false,
            Some((_, right)) => point<=right,
        }){
            InInterval
        } else {
            let option = self.points.get(point);
            match option {
                Some(Status::Include) => InInclude,
                Some(Status::Exclude) => InExclude,
                None => Outside,
            }
        }
    }
}

impl<T: Ord+Clone> ContinuedSets<T>{
    pub fn add_interval(&mut self,interval: Interval<T>){
        let (itv_lf,itv_rt,flag)=interval.take_away();
        let left_status=self.classify_point(&itv_lf);
        let right_status=self.classify_point(&itv_rt);

        let new_left =match(self.intervals.left_of(&itv_lf)){
            Some((left,right)) if(&itv_lf<=right)=> {
                match self.intervals.remove_entry(&left.clone()) {
                    None => {itv_lf}
                    Some((left,_)) => {left}
                }
            }
            _ => {itv_lf}
        };

        let new_right =match(self.intervals.left_of(&itv_rt)){
            Some((left,right)) if(&itv_rt<=right)=> {
                match self.intervals.remove_entry(&left.clone()) {
                    None => {itv_rt}
                    Some((_,right)) => {right}
                }
            }
            _ => {itv_rt}
        };
        
        let keys_to_delete=self.intervals.collect_keys(&new_left..=&new_right);
        for key in keys_to_delete {
            self.intervals.remove(&key);
        }
        self.intervals.insert(new_left.clone(), new_right.clone());
        match left_status {
            InInclude => {
                if(flag.left_open()){
                    self.remove_point(new_left);
                }
            }
            InExclude => {
                if(!flag.left_open()){
                    self.add_point(new_left);
                }
            }
            _=>{}
        }
        match right_status {
            InInclude => {
                if(flag.right_open()){
                    self.remove_point(new_right);
                }
            }
            InExclude => {
                if(!flag.right_open()){
                    self.add_point(new_right);
                }
            }
            _=>{}
        }
        
    }
    
}


enum PointStatus {
    InInclude,
    InExclude,
    InInterval,
    Outside,
}

#[cfg(test)]
mod tests {

}