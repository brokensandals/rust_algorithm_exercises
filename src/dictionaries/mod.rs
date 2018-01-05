mod unsorted_array;

pub trait Cursor<K: Ord, V> {
    fn delete(&self);
    fn predecessor(self) -> Option<Self>;
    fn successor(self) -> Option<Self>;
    fn key(&self) -> &K;
    fn value(&self) -> Option<&V>;
    fn set_value(&self, value: V);
}

pub trait Dictionary<K: Ord, V> {
    fn search(self, key: K) -> Cursor<K, V>;
    fn max(&self) -> Option<Cursor<K, V>>;
    fn min(&self) -> Option<Cursor<K, V>>;
}
