use ::dictionaries::Dictionary;
use ::dictionaries::Cursor;

struct Entry<K, V> {
    key: K,
    value: V,
}

pub struct UnsortedArrayDictionary<K: Ord, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K: Ord, V> UnsortedArrayDictionary<K, V> {
    fn new() -> UnsortedArrayDictionary<K, V> {
        UnsortedArrayDictionary { entries: Vec::new() }
    }

    fn cursor(&self, key: K, index: Option<usize>) -> InternalCursor<K, V> {
        InternalCursor {
            dict: self,
            key: key,
            index: index,
        }
    }
}

impl<K: Ord, V> Dictionary<K, V> for UnsortedArrayDictionary<K, V> {
    fn search(&self, key: K) -> InternalCursor<K, V> {
        self.cursor(key, None) // TODO
    }

    fn max(&self) -> Option<InternalCursor<K, V>> {
        None // TODO
    }

    fn min(&self) -> Option<InternalCursor<K, V>> {
        None // TODO
    }
}

struct InternalCursor<K: Ord, V> {
    dict: &UnsortedArrayDictionary<K, V>,
    key: &K,
    index: Option<usize>,
}

impl<K: Ord, V> Cursor<K, V> for InternalCursor<K, V> {
    fn delete(&self) {
        // TODO
    }

    fn predecessor(self) -> Option<Self> {
        None // TODO
    }

    fn successor(self) -> Option<Self> {
        None // TODO
    }

    fn key(&self) -> &K {
        self.key // TODO
    }

    fn value(&self) -> Option<&V> {
        None // TODO
    }

    fn set_value(&self, value: V) {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn tuple<K: Eq + Ord + Copy, V: Copy>(entry: &Entry<K, V>) -> (K, V) {
    //     (entry.key, entry.value)
    // }

    // #[test]
    // fn it_performs_insertions_and_searches() {
    //     let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
    //     dict.insert(Entry { key: 2, value: 200 });
    //     dict.insert(Entry { key: 1, value: 100 });
    //     dict.insert(Entry { key: 3, value: 300 });
    //     assert_eq!(Some((1, 100)), dict.search(1).map(tuple));
    //     assert_eq!(Some((2, 200)), dict.search(2).map(tuple));
    //     assert_eq!(Some((3, 300)), dict.search(3).map(tuple));
    //     assert_eq!(None, dict.search(4).map(tuple));
    // }

    // #[test]
    // fn it_performs_deletions() {
    //     let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
    //     dict.insert(1, 100);
    //     assert_eq!(Some(100), dict.search(1));
    //     dict.delete(1);
    //     assert_eq!(None, dict.search(1));
    // }

    // #[test]
    // fn it_returns_min_and_max_elements() {
    //     let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
    //     assert_eq!(None, dict.min());
    //     assert_eq!(None, dict.max());
    //     dict.insert(2, 200);
    //     assert_eq!(Some(2), dict.min());
    //     assert_eq!(Some(2), dict.max());
    //     dict.insert(1, 100);
    //     dict.insert(3, 300);
    //     assert_eq!(Some(1), dict.min());
    //     assert_eq!(Some(3), dict.max());
    // }

    // #[test]
    // fn it_returns_predecessors() {
    //     let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
    //     assert_eq!(None, dict.predecessor(2));
    //     dict.insert(2, 200);
    //     dict.insert(1, 100);
    //     dict.insert(3, 300);
    //     assert_eq!(None, dict.predecessor(1));
    //     assert_eq!(Some(1), dict.predecessor(2));
    //     assert_eq!(Some(2), dict.predecessor(3));
    // }

    // #[test]
    // fn it_returns_successors() {
    //     let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
    //     assert_eq!(None, dict.successor(2));
    //     dict.insert(2, 200);
    //     dict.insert(1, 100);
    //     dict.insert(3, 300);
    //     assert_eq!(Some(2), dict.successor(1));
    //     assert_eq!(Some(3), dict.successor(2));
    //     assert_eq!(None, dict.successor(3));
    // }
}
