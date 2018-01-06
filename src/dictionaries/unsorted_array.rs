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
    pub fn new() -> UnsortedArrayDictionary<K, V> {
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
    extern crate test;

    use super::*;
    use super::super::test_util::*;

    dictionary_tests!(UnsortedArrayDictionary);
}
