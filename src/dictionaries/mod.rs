use std::marker::Sized;

mod unsorted_array;

/// A dictionary for storing key-value pairs and searching/traversing by key.
/// The definition of the operations is influenced by the definition of
/// dictionary given in The Algorithm Design Manual, 2nd Edition - specifically,
/// it's designed so that if you search for a key, you can delete it or
/// find its successor or predecessor without the dictionary having to
/// duplicate the effort of searching for that key. Unlike the referenced book,
/// which has the search operation return a pointer to the entry, here the
/// search operation returns a cursor object which implements all operations
/// on the entry.
pub trait Dictionary<'d, K: Ord + 'd, V: 'd> {
    type Cursor: Cursor<'d, K, V>;

    /// Get a cursor for inserting, updating, retrieving, or deleting
    /// the value at the specified key.
    fn search(&'d mut self, key: K) -> Self::Cursor;

    /// Get a cursor for the entry with the highest key in the dictionary,
    /// or None if the dictionary is empty.
    fn max(&'d mut self) -> Option<Self::Cursor>;

    /// Get a cursor for the entry with the lowest key in the dictionary,
    /// or None if the dictionary is empty.
    fn min(&'d mut self) -> Option<Self::Cursor>;
}

pub trait Cursor<'d, K: Ord + 'd, V: 'd> where Self: Sized {
    /// Delete this entry, or do nothing if not currently pointing to an entry.
    /// The cursor remains valid after this operation.
    fn delete(&mut self);

    /// Get a cursor to the entry whose key is previous to this one in sorted order,
    /// or None if there is none.
    fn predecessor(self) -> Option<Self>;

    /// Get a cursor to the entry whose key is subsequent to this one in sorted order,
    /// or None if there is none.
    fn successor(self) -> Option<Self>;

    /// Get the key this cursor points to.
    fn key(&self) -> &K;

    /// Get the value for this key in the dictionary, or None if the key is
    /// not present.
    fn value(&self) -> Option<&V>;

    /// Set the value for this key in the dictionary. This will overwrite an
    /// existing value for the key if one exists.
    fn set_value(&mut self, value: V);
}
