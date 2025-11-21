use std::collections::{BTreeMap, HashMap};
use std::ops::Index;

pub trait Map<K,V>{
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn contains_key(&self, key: &K) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn keys(&self) -> Vec<&K>;
    fn values(&self) -> Vec<&V>;
    fn all(&self) -> Vec<(&K,&V)>;
}
impl<K: std::cmp::Eq + std::hash::Hash,V> Map<K,V> for HashMap<K,V>{
    fn get(&self, key: &K) -> Option<&V>{
         HashMap::get(self, key)
    }
    fn get_mut(&mut self, key: &K) -> Option<&mut V>{
        HashMap::get_mut(self,key)
    }
    fn insert(&mut self, key: K, value: V) -> Option<V>{
        HashMap::insert(self,key,value)
    }
    fn remove(&mut self, key: &K) -> Option<V>{
        HashMap::remove(self, key)
    }
    fn contains_key(&self, key: &K) -> bool{
        HashMap::contains_key(self, key)
    }
    fn len(&self) -> usize{
        HashMap::len(self)
    }
    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }
    fn keys(&self) -> Vec<&K> {
        HashMap::keys(self).collect()
    }
    fn values(&self) -> Vec<&V> {
        HashMap::values(self).collect()
    }
    fn all(&self) -> Vec<(&K,&V)>{
        self.iter().collect()
    }
}
impl<K: std::cmp::Eq + std::hash::Hash + std::cmp::Ord,V> Map<K, V> for BTreeMap<K,V>{
    fn get(&self, key: &K) -> Option<&V>{
        BTreeMap::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        BTreeMap::get_mut(self, key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        BTreeMap::insert(self,key,value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        BTreeMap::remove(self, key)
    }

    fn contains_key(&self, key: &K) -> bool {
        BTreeMap::contains_key(self, key)
    }

    fn len(&self) -> usize {
        BTreeMap::len(self)
    }

    fn is_empty(&self) -> bool {
        BTreeMap::is_empty(self)
    }

    fn keys(&self) -> Vec<&K> {
        BTreeMap::keys(self).collect()
    }

    fn values(&self) -> Vec<&V> {
        BTreeMap::values(self).collect()
    }

    fn all(&self) -> Vec<(&K, &V)> {
        BTreeMap::all(self)
    }
}