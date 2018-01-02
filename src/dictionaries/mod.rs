mod unsorted_array;

pub trait Cursor<K, V> {
    fn delete(&self);
    fn predecessor(self) -> Self;
    fn successor(self) -> Self;
    fn key(&self) -> &K;
    fn value(&self) -> Option<&V>;
    fn set_value(&self, value: V);
}

pub trait Dictionary<K: Eq + Ord, V> {
    type C: Cursor<K, V>;

    fn search(self, key: K) -> Self::C;
    fn max(&self) -> Self::C;
    fn min(&self) -> Self::C;
}
