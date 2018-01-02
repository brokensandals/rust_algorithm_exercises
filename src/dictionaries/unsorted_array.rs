use ::dictionaries::Dictionary;
use ::dictionaries::Entry;

pub struct UnsortedArrayDictionary<K: Eq + Ord, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K: Eq + Ord, V> UnsortedArrayDictionary<K, V> {
    fn new() -> UnsortedArrayDictionary<K, V> {
        UnsortedArrayDictionary { entries: Vec::new() }
    }
}

impl<K: Eq + Ord, V> Dictionary<K, V> for UnsortedArrayDictionary<K, V> {
    fn search(&self, key: K) -> Option<&Entry<K, V>> {
        self.entries.iter().find(|&entry| entry.key == key)
    }

    fn insert(&mut self, entry: Entry<K, V>) {
        self.entries.push(entry);
    }

    fn delete(&mut self, entry: &Entry<K, V>) {
        
    }

    fn max(&self) -> Option<&Entry<K, V>> {
        None
    }

    fn min(&self) -> Option<&Entry<K, V>> {
        None
    }

    fn predecessor(&self, entry: &Entry<K, V>) -> Option<&Entry<K, V>> {
        None
    }

    fn successor(&self, entry: &Entry<K, V>) -> Option<&Entry<K, V>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tuple<K: Eq + Ord + Copy, V: Copy>(entry: &Entry<K, V>) -> (K, V) {
        (entry.key, entry.value)
    }

    #[test]
    fn it_performs_insertions_and_searches() {
        let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
        dict.insert(Entry { key: 2, value: 200 });
        dict.insert(Entry { key: 1, value: 100 });
        dict.insert(Entry { key: 3, value: 300 });
        assert_eq!(Some((1, 100)), dict.search(1).map(tuple));
        assert_eq!(Some((2, 200)), dict.search(2).map(tuple));
        assert_eq!(Some((3, 300)), dict.search(3).map(tuple));
        assert_eq!(None, dict.search(4).map(tuple));
    }

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
