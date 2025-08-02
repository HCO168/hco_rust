use std::collections::{BTreeMap, BTreeSet};
use crate::math::interval::Interval;

pub trait OrderedSetSearch<T: Ord> {
    fn left_of(&self, value: &T) -> Option<&T>;       // 前驱
    fn right_of(&self, value: &T) -> Option<&T>;      // 后继
    fn neighbors(&self, value: &T) -> (Option<&T>, Option<&T>); // 同时返回前驱+后继
}

impl<T: Ord> OrderedSetSearch<T> for BTreeSet<T> {
    fn left_of(&self, value: &T) -> Option<&T> {
        self.range(..=value).next_back()
    }

    fn right_of(&self, value: &T) -> Option<&T> {
        self.range(value..).next()
    }

    fn neighbors(&self, value: &T) -> (Option<&T>, Option<&T>) {
        (self.left_of(value), self.right_of(value))
    }
}
pub trait OrderedMapSearch<K: Ord, V> {
    fn left_of(&self, key: &K) -> Option<(&K, &V)>;       // 前驱
    fn right_of(&self, key: &K) -> Option<(&K, &V)>;      // 后继
    fn neighbors(&self, key: &K) -> (Option<(&K, &V)>, Option<(&K, &V)>); // 同时返回前驱+后继
}

pub trait OrderedMapCollect<K: Ord+Clone, V> {

    fn collect_keys_ref(&self, interval: impl std::ops::RangeBounds<K>) -> Vec<&K>;

    fn collect_keys(&self, interval: impl std::ops::RangeBounds<K>) -> Vec<K>;
}
impl<K: Ord, V> OrderedMapSearch<K, V> for BTreeMap<K, V> {
    fn left_of(&self, key: &K) -> Option<(&K, &V)> {
        self.range(..=key).next_back()
    }

    fn right_of(&self, key: &K) -> Option<(&K, &V)> {
        self.range(key..).next()
    }

    fn neighbors(&self, key: &K) -> (Option<(&K, &V)>, Option<(&K, &V)>) {
        (self.left_of(key), self.right_of(key))
    }
}

impl<K: Ord+Clone, V> OrderedMapCollect<K, V> for BTreeMap<K, V> {
    fn collect_keys_ref(&self, interval: impl std::ops::RangeBounds<K>) -> Vec<&K> {
        self.range(interval).map(|(k, _)| k).collect()
    }

    fn collect_keys(&self, interval: impl std::ops::RangeBounds<K>) -> Vec<K> {
        self.range(interval).map(|(k, _)|->K{k.clone()}).collect()
    }
}
#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use crate::containers::ordered::*;

    #[test]
    fn test_left_of() {
        let set = BTreeSet::from_iter([1, 2, 7, 8, 9, 10, 14, 15, 16]);
        assert_eq!(set.range(..=13).next_back(), Some((&10)));
    }
    #[test]
    fn test_borrow(){
    }
    #[test]
    fn test_remove_by_ref_with_string_key() {
        // 构造一个 BTreeMap，key 是不可 Copy 的 String
        let mut map: BTreeMap<String, &str> =
            ["aa", "ab", "ac"].into_iter().map(|x| (x.to_string(), "val")).collect();

        unsafe {
            // 获取 key 的引用 (&String)，这是不可 Copy 类型
            let key_ref = map.first_key_value().unwrap().0;
            let raw_ptr: *const String = key_ref;

            // 通过 unsafe 方法移除该 entry
            let val = map.remove(&*raw_ptr);
            assert_eq!(val, Some("val"));

            // ⚠️ 删除后不要再使用 key_ref！
        }

        assert_eq!(map.len(), 2); // 确认删除生效
    }

}