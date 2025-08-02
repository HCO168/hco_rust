use std::cmp::Ordering;
use std::cmp::Ordering::Greater;
use std::{mem, ptr};
use std::mem::{swap, MaybeUninit};
use crate::containers::array::Array;

pub fn merge_sort<T>(l:&mut [T], a:usize, b:usize,cmp:fn(&T,&T)->Ordering)
where T:{
    //when only 1 or no element, no need to sort
    if(b-a<=1){
        return;
    }
    //create a temp array to store
    let mut tmp=unsafe{Array::<MaybeUninit<T>>::with_uninit(b-a)};
    merge_sort_recur(l,a,b,&mut tmp,cmp);
}
pub fn merge_sort_recur<T>(l:&mut [T], a:usize, b:usize, tmp:&mut [MaybeUninit<T>], cmp:fn(&T,&T)->Ordering)
where T:{
    let len=b-a;
    //when only 1 or no element, no need to sort
    if(b-a<=1){
        return;
    }
    //goal: make all cmp return less or equal
    if(len==2&&cmp(&l[a],&l[a+1])==Greater){
        l.swap(a,a+1);
        return;
    }
    /*
    if(len<16){
        insertion_sort(l,a,b);
        return;
    }
    */
    let m:usize=a+len/2;
    merge_sort_recur(l,a,m,tmp,cmp);
    merge_sort_recur(l,m,b,tmp,cmp);
    merge(l,a,m,b,tmp,cmp);
}
pub fn merge<T>(l:&mut[T],a:usize,m:usize,b:usize,tmp:&mut [MaybeUninit<T>],cmp:fn(&T,&T)->Ordering)
where T:
{
    let mut i_left = a;
    let mut i_right = m;
    let mut i_tmp = 0;
    unsafe {
        while (i_left < m && i_right < b) {
            //less/equal
            if (cmp(&l[i_left], &l[i_right])!=Greater) {
                set_uninit(&mut tmp[i_tmp], &mut l[i_left]);
                i_left += 1;
                i_tmp += 1;
            } else {
                set_uninit(&mut tmp[i_tmp], &mut l[i_right]);
                i_right += 1;
                i_tmp += 1;
            }
        }
        while (i_left < m) {
            set_uninit(&mut tmp[i_tmp], &mut l[i_left]);
            i_left += 1;
            i_tmp += 1;
        }
        while (i_right < b) {
            set_uninit(&mut tmp[i_tmp], &mut l[i_right]);
            i_right += 1;
            i_tmp += 1;
        }
        i_tmp = a;
        while (i_tmp < b) {
            set_init_from_uninit(&mut l[i_tmp], &mut tmp[i_tmp]);
            i_tmp = i_tmp + 1;
        }
    }
}
pub fn insertion_sort<T>(l:&mut [T],a:usize,b:usize,cmp:fn(&T,&T)->Ordering)where T:PartialOrd{
    for i in a + 1..b {
        let mut j = i;
        while j > a && cmp(&l[j - 1], &l[j]) == Greater {
            l.swap(j - 1, j);
            j -= 1;
        }
    }
}
unsafe fn set_uninit<T>(dst: &mut MaybeUninit<T>,src: &mut T) {
    // SAFETY: src 必须有效且不会再使用，dst 必须未初始化
    let val = ptr::read(src);              // move 出 src
    ptr::write(dst.as_mut_ptr(), val);   
    // 写入 dst
}
unsafe fn set_init_from_uninit<T>(dst: &mut T,src: &mut MaybeUninit<T>) {
    // SAFETY: src 必须已初始化且不会再使用
    let val = ptr::read(src.as_ptr());          // move 出 src
    ptr::write(dst, val);
    // 写入 dst
}
