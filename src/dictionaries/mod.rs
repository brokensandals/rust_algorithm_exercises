mod unsorted_array;

pub struct Entry<K: Eq + Ord, V> {
    key: K,
    value: V,
}

pub trait Dictionary<K: Eq + Ord, V> {
    fn search(&self, key: K) -> Option<&Entry<K, V>>;
    fn insert(&mut self, entry: &Entry<K, V>);
    fn delete(&mut self, entry: &Entry<K, V>);
    fn max(&self) -> Option<&Entry<K, V>>;
    fn min(&self) -> Option<&Entry<K, V>>;
    fn predecessor(&self, entry: &Entry<K, V>) -> Option<&Entry<K, V>>;
    fn successor(&self, entry: &Entry<K, V>) -> Option<&Entry<K, V>>;
}
