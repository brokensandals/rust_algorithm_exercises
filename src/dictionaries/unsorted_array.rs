use ::dictionaries::Dictionary;

struct Entry<K, V> {
    key: K,
    value: V,
}

pub struct UnsortedArrayDictionary<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K, V> Dictionary<K, V> for UnsortedArrayDictionary<K, V> {
    fn search(&self, key: K) -> Option<V> {
        None
    }

    fn insert(&mut self, key: K, value: V) {

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
