//generate by ChatGPT

use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Index, IndexMut, Deref, DerefMut};

pub struct Array<T> {
    data: Box<[T]>,
}

#[macro_export]
macro_rules! array {
    // 重复值初始化: array![value; len]
    ($val:expr; $len:expr) => {{
        $crate::Array::from_value($len, $val)
    }};
    
    // 列表初始化: array![v1, v2, v3, ...]
    ($($elem:expr),* $(,)?) => {{
        $crate::Array::from_iter([$($elem),*])
    }};
}

impl<T> Array<T> {
    
    /// 从迭代器创建（可存任意类型，长度由输入决定）
    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Array {
            data: iter.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    /// 用克隆值初始化（需 Clone）
    pub fn from_value(len: usize, value: T) -> Self
    where
        T: Clone,
    {
        Array {
            data: std::iter::repeat(value).take(len).collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    /// 通过 Vec 初始化
    pub fn from_vec(vec: Vec<T>) -> Self {
        Array {
            data: vec.into_boxed_slice(),
        }
    }

    /// 通过切片（包括数组字面量）初始化
    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        Array {
            data: slice.to_vec().into_boxed_slice(),
        }
    }

    /// 长度
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
}

// 索引访问
impl<T: Default> Array<T> {
    /// 创建指定长度的 Array，并用 `T::default()` 初始化每个元素
    pub fn with_len(len: usize) -> Self {
        let data = (0..len)
            .map(|_| T::default())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Array { data }
    }
}

impl<T> Array<MaybeUninit<T>> {
    /// 创建一个未初始化的 Array<MaybeUninit<T>>
    pub unsafe fn with_uninit(len: usize) -> Self {
        let boxed: Box<[MaybeUninit<T>]> = unsafe {Box::new_uninit_slice(len).assume_init()};
        Array { data: boxed }
    }

    /// 安全地转为已初始化数组（需确保所有元素已正确写入）
    pub unsafe fn assume_init(self) -> Array<T> {
        let raw = Box::into_raw(self.data) as *mut [T];
        Array {
            data: unsafe{Box::from_raw(raw)},
        }
    }
}



impl<T> Index<usize> for Array<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// 解引用成切片
impl<T> Deref for Array<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

// Debug 打印
impl<T: std::fmt::Debug> std::fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}
