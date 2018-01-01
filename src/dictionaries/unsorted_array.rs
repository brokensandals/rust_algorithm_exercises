use ::dictionaries::Dictionary;

struct Entry<K: Eq, V: Copy> {
    key: K,
    value: V,
}

pub struct UnsortedArrayDictionary<K: Eq, V: Copy> {
    entries: Vec<Entry<K, V>>,
}

impl<K: Eq, V: Copy> UnsortedArrayDictionary<K, V> {
    fn new() -> UnsortedArrayDictionary<K, V> {
        UnsortedArrayDictionary { entries: Vec::new() }
    }
}

impl<K: Eq, V: Copy> Dictionary<K, V> for UnsortedArrayDictionary<K, V> {
    fn search(&self, key: K) -> Option<V> {
        for entry in &self.entries {
            if entry.key == key {
                return Some(entry.value);
            }
        }
        None
    }

    fn insert(&mut self, key: K, value: V) {
        self.entries.push(Entry { key: key, value: value });
    }

    fn delete(&mut self, key: K) {
        match self.entries.iter().position(|ref entry| entry.key == key) {
            Some(index) => { self.entries.remove(index); },
            None =>  { },
        };
    }

    fn max(&self) -> Option<K> {
        None
    }

    fn min(&self) -> Option<K> {
        None
    }

    fn predecessor(&self, key: K) -> Option<K> {
        None
    }

    fn successor(&self, key: K) -> Option<K> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_performs_insertions_and_searches() {
        let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
        dict.insert(2, 200);
        dict.insert(1, 100);
        dict.insert(3, 300);
        assert_eq!(Some(100), dict.search(1));
        assert_eq!(Some(200), dict.search(2));
        assert_eq!(Some(300), dict.search(3));
        assert_eq!(None, dict.search(4));
    }

    #[test]
    fn it_performs_deletions() {
        let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
        dict.insert(1, 100);
        assert_eq!(Some(100), dict.search(1));
        dict.delete(1);
        assert_eq!(None, dict.search(1));
    }

    #[test]
    fn it_returns_min_and_max_elements() {
        let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
        assert_eq!(None, dict.min());
        assert_eq!(None, dict.max());
        dict.insert(2, 200);
        assert_eq!(Some(200), dict.min());
        assert_eq!(Some(200), dict.max());
        dict.insert(1, 100);
        dict.insert(3, 300);
        assert_eq!(Some(100), dict.min());
        assert_eq!(Some(300), dict.max());
    }
}
