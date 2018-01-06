use ::dictionaries::Dictionary;
use ::dictionaries::Cursor;
use self::CursorState::*;
use std::mem;

struct Entry<K, V> {
    key: K,
    value: V,
}

pub struct SortedArrayDictionary<K: Ord, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K: Ord, V> SortedArrayDictionary<K, V> {
    pub fn new() -> SortedArrayDictionary<K, V> {
        SortedArrayDictionary { entries: Vec::new() }
    }
}

impl<'d, K: Ord + 'd, V: 'd> Dictionary<'d, K, V> for SortedArrayDictionary<K, V> {
    type Cursor = SortedArrayDictionaryCursor<'d, K, V>;

    fn search(&'d mut self, key: K) -> Self::Cursor {
        match self.entries.binary_search_by(|ref e| e.key.cmp(&key)) {
            Ok(index) => SortedArrayDictionaryCursor {
                dictionary: self,
                index: index,
                state: PresentElement,
            },
            Err(index) => SortedArrayDictionaryCursor {
                dictionary: self,
                index: index,
                state: MissingElement(key),
            },
        }
    }

    fn max(&'d mut self) -> Option<Self::Cursor> {
        match self.entries.len() {
            0 => None,
            len => Some(SortedArrayDictionaryCursor {
                dictionary: self,
                index: len - 1,
                state: PresentElement,
            }),
        }
    }

    fn min(&'d mut self) -> Option<Self::Cursor> {
        match self.entries.len() {
            0 => None,
            _ => Some(SortedArrayDictionaryCursor {
                dictionary: self,
                index: 0,
                state: PresentElement,
            }),
        }
    }
}    

enum CursorState<K> {
    MissingElement(K),
    PresentElement,
}

pub struct SortedArrayDictionaryCursor<'d, K: Ord + 'd, V: 'd> {
    dictionary: &'d mut SortedArrayDictionary<K, V>,
    index: usize,
    state: CursorState<K>,
}

impl<'d, K: Ord + 'd, V: 'd> Cursor<'d, K, V> for SortedArrayDictionaryCursor<'d, K, V> {
    fn delete(&mut self) {
        match &self.state {
            &MissingElement(_) => { },
            &PresentElement => {
                let entry = self.dictionary.entries.remove(self.index);
                self.state = MissingElement(entry.key);
            },
        };
    }

    fn predecessor(self) -> Option<Self> {
        if self.index == 0 {
            return None;
        }

        Some(SortedArrayDictionaryCursor {
            dictionary: self.dictionary,
            index: self.index - 1,
            state: PresentElement,
        })
    }

    fn successor(self) -> Option<Self> {
        match self.state {
            MissingElement(_) if self.index == self.dictionary.entries.len() => None,
            MissingElement(_) => Some(SortedArrayDictionaryCursor {
                dictionary: self.dictionary,
                index: self.index,
                state: PresentElement,
            }),
            PresentElement if self.index == self.dictionary.entries.len() - 1 => None,
            PresentElement => Some(SortedArrayDictionaryCursor {
                dictionary: self.dictionary,
                index: self.index + 1,
                state: PresentElement,
            }),
        }
    }

    fn key(&self) -> &K {
        match &self.state {
            &MissingElement(ref key) => key,
            &PresentElement => &self.dictionary.entries[self.index].key,
        }
    }

    fn value(&self) -> Option<&V> {
        match &self.state {
            &MissingElement(_) => None,
            &PresentElement => Some(&self.dictionary.entries[self.index].value),
        }
    }

    fn set_value(&mut self, value: V) {
        match &self.state {
            &MissingElement(_) => {
                match mem::replace(&mut self.state, PresentElement) {
                    MissingElement(key) => { self.dictionary.entries.insert(self.index, Entry { key: key, value: value }); },
                    PresentElement => unreachable!(),
                };
            },
            &PresentElement => {
                self.dictionary.entries[self.index].value = value;
            },
        };
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use super::super::test_util::*;

    dictionary_tests!(SortedArrayDictionary);
}
