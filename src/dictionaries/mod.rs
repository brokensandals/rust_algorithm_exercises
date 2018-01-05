use std::marker::Sized;

mod unsorted_array;

pub trait Cursor<'d, K: Ord + 'd, V: 'd> where Self: Sized {
    fn delete(&mut self);
    fn predecessor(self) -> Option<Self>;
    fn successor(self) -> Option<Self>;
    fn key(&self) -> &K;
    fn value(&self) -> Option<&V>;
    fn set_value(&mut self, value: V);
}

pub trait Dictionary<'d, K: Ord + 'd, V: 'd> {
    type Cursor: Cursor<'d, K, V>;

    fn search(&'d mut self, key: K) -> Self::Cursor;
    fn max(&self) -> Option<Self::Cursor>;
    fn min(&self) -> Option<Self::Cursor>;
}
