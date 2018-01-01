mod unsorted_array;

pub trait Dictionary<K: Eq, V: Copy> {
    fn search(&self, key: K) -> Option<V>;
    fn insert(&mut self, key: K, value: V);
    fn delete(&mut self, key: K);
    fn max(&self) -> Option<K>;
    fn min(&self) -> Option<K>;
    fn predecessor(&self, key: K) -> Option<K>;
    fn successor(&self, key: K) -> Option<K>;
}
