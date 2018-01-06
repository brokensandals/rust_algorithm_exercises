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

    fn max(&'d mut self) -> Option<Self::Cursor> {
        if self.entries.is_empty() {
            return None;
        }

        let mut index = 0;
        {
            let mut max = &self.entries[0].key;
            for i in 1..(self.entries.len()) {
                if *max < self.entries[i].key {
                    max = &self.entries[i].key;
                    index = i;
                }
            }
        }

        Some(UnsortedArrayDictionaryCursor {
            dictionary: self,
            position: PresentElement(index),
        })
    }

    fn min(&'d mut self) -> Option<Self::Cursor> {
        if self.entries.is_empty() {
            return None;
        }

        let mut index = 0;
        {
            let mut min = &self.entries[0].key;
            for i in 1..(self.entries.len()) {
                if *min > self.entries[i].key {
                    min = &self.entries[i].key;
                    index = i;
                }
            }
        }

        Some(UnsortedArrayDictionaryCursor {
            dictionary: self,
            position: PresentElement(index),
        })
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
    fn delete(&mut self) {
        match &self.position {
            &MissingElement(_) => { },
            &PresentElement(index) => {
                let entry = self.dictionary.entries.swap_remove(index);
                self.position = MissingElement(entry.key);
            }
        };
    }

    fn predecessor(self) -> Option<Self> {
        let mut index: Option<usize> = None;
        {
            let mut max: Option<&K> = None;
            for i in 0..(self.dictionary.entries.len()) {
                let key = &self.dictionary.entries[i].key;
                if *key < *self.key() {
                    if max.is_none() || *max.unwrap() < *key {
                        index = Some(i);
                        max = Some(key);
                    }
                }
            }
        }

        match index {
            None => None,
            Some(index) => Some(UnsortedArrayDictionaryCursor {
                dictionary: self.dictionary,
                position: PresentElement(index),
            }),
        }
    }

    fn successor(self) -> Option<Self> {
        let mut index: Option<usize> = None;
        {
            let mut min: Option<&K> = None;
            for i in 0..(self.dictionary.entries.len()) {
                let key = &self.dictionary.entries[i].key;
                if *key > *self.key() {
                    if min.is_none() || *min.unwrap() > *key {
                        index = Some(i);
                        min = Some(key);
                    }
                }
            }
        }

        match index {
            None => None,
            Some(index) => Some(UnsortedArrayDictionaryCursor {
                dictionary: self.dictionary,
                position: PresentElement(index),
            }),
        }
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

    #[test]
    fn it_performs_insertions_and_searches() {
        let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
        dict.search(2).set_value(200);
        dict.search(1).set_value(100);
        dict.search(3).set_value(300);
        assert_eq!(Some(100), dict.search(1).value().cloned());
        assert_eq!(Some(200), dict.search(2).value().cloned());
        assert_eq!(Some(300), dict.search(3).value().cloned());
        assert_eq!(None, dict.search(4).value());
    }

    #[test]
    fn it_performs_deletions() {
        let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
        dict.search(1).set_value(100);
        assert_eq!(Some(100), dict.search(1).value().cloned());
        dict.search(1).delete();
        assert_eq!(None, dict.search(1).value());
    }

    #[test]
    fn it_returns_min_and_max_elements() {
        let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
        assert!(dict.min().is_none());
        assert!(dict.max().is_none());
        dict.search(2).set_value(200);
        assert!(dict.min().is_some());
        assert!(dict.max().is_some());
        assert_eq!(Some(200), dict.min().unwrap().value().cloned());
        assert_eq!(Some(200), dict.max().unwrap().value().cloned());
        dict.search(1).set_value(100);
        dict.search(3).set_value(300);
        assert!(dict.min().is_some());
        assert!(dict.max().is_some());
        assert_eq!(Some(100), dict.min().unwrap().value().cloned());
        assert_eq!(Some(300), dict.max().unwrap().value().cloned());
    }

    #[test]
    fn it_returns_predecessors() {
        let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
        dict.search(2).set_value(200);
        assert!(dict.search(2).predecessor().is_none());
        dict.search(1).set_value(100);
        assert!(dict.search(2).predecessor().is_some());
        assert_eq!(Some(100), dict.search(2).predecessor().unwrap().value().cloned());
    }

    #[test]
    fn it_returns_successors() {
        let mut dict: UnsortedArrayDictionary<i64, i64> = UnsortedArrayDictionary::new();
        dict.search(2).set_value(200);
        assert!(dict.search(2).successor().is_none());
        dict.search(3).set_value(300);
        assert!(dict.search(2).successor().is_some());
        assert_eq!(Some(300), dict.search(2).successor().unwrap().value().cloned());
    }
}
