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
    }
}
