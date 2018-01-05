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
    type Cursor: Cursor<K, V>;

    fn search(self, key: K) -> Self::Cursor;
    fn max(&self) -> Option<Self::Cursor>;
    fn min(&self) -> Option<Self::Cursor>;
}
