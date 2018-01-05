use ::dictionaries::Dictionary;
use ::dictionaries::Cursor;
use self::CursorPosition::*;
use std::mem;

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
}

impl<'d, K: Ord + 'd, V: 'd> Dictionary<'d, K, V> for UnsortedArrayDictionary<K, V> {
    type Cursor = UnsortedArrayDictionaryCursor<'d, K, V>;

    fn search(&'d mut self, key: K) -> Self::Cursor {
        match self.entries.iter().position(|ref e| e.key == key) {
            None => UnsortedArrayDictionaryCursor {
                dictionary: self,
                position: MissingElement(key),
            },
            Some(position) => UnsortedArrayDictionaryCursor {
                dictionary: self,
                position: PresentElement(position),
            },
        }
    }

    fn max(&self) -> Option<Self::Cursor> {
        None // TODO
    }

    fn min(&self) -> Option<Self::Cursor> {
        None // TODO
    }
}

enum CursorPosition<K> {
    MissingElement(K),
    PresentElement(usize),
}

pub struct UnsortedArrayDictionaryCursor<'d, K: Ord + 'd, V: 'd> {
    dictionary: &'d mut UnsortedArrayDictionary<K, V>,
    position: CursorPosition<K>,
}

impl<'d, K: Ord + 'd, V: 'd> Cursor<'d, K, V> for UnsortedArrayDictionaryCursor<'d, K, V> {
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
        match &self.position {
            &MissingElement(ref key) => key,
            &PresentElement(index) => &self.dictionary.entries[index].key,
        }
    }

    fn value(&self) -> Option<&V> {
        match &self.position {
            &MissingElement(_) => None,
            &PresentElement(index) => Some(&self.dictionary.entries[index].value),
        }
    }

    fn set_value(&mut self, value: V) {
        match &self.position {
            &MissingElement(_) => {
                match mem::replace(&mut self.position, PresentElement(self.dictionary.entries.len())) {
                    MissingElement(key) => { self.dictionary.entries.push(Entry { key: key, value: value }); },
                    PresentElement(_) => unreachable!(),
                };
            },
            &PresentElement(index) => {
                self.dictionary.entries[index].value = value;
            },
        };
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
