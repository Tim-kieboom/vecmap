use std::marker::PhantomData;

use crate::{vec_map::VecMap, vec_map_index::VecMapIndex};

impl<I: VecMapIndex, T> VecMap<I, T> {
    /// Returns an iterator over existing `index`.
    pub fn keys(&self) -> VecMapKeysIter<'_, I, T> {
        VecMapKeysIter::new(self.vec.iter().enumerate())
    }

    /// Returns an iterator over existing `(index, &value)` pairs.
    pub fn entries(&self) -> VecMapEntriesIter<'_, I, T> {
        VecMapEntriesIter::new(self.vec.iter().enumerate())
    }

    /// Consumes the map and returns an iterator over `(index, value)` pairs.
    pub fn into_entries(self) -> VecMapEntriesIntoIter<I, T> {
        VecMapEntriesIntoIter::new(self.vec.into_iter().enumerate())
    }

    /// Returns an iterator over references to all existing values.
    pub fn values(&self) -> VecMapValuesIter<'_, T> {
        self.vec.iter().flatten()
    }

    /// Returns an iterator over references to all existing values.
    pub fn values_mut(&mut self) -> VecMapValuesIterMut<'_, T> {
        self.vec.iter_mut().flatten()
    }

    /// Consumes the map and returns an iterator over all values.
    pub fn into_values(self) -> VecMapValuesIntoIter<T> {
        self.vec.into_iter().flatten()
    }
}

pub type VecMapValuesIter<'a, T> = std::iter::Flatten<std::slice::Iter<'a, Option<T>>>;
pub type VecMapValuesIterMut<'a, T> = std::iter::Flatten<std::slice::IterMut<'a, Option<T>>>;
pub type VecMapValuesIntoIter<T> = std::iter::Flatten<std::vec::IntoIter<Option<T>>>;

impl<I: VecMapIndex, T> FromIterator<(I, T)> for VecMap<I, T> {
    fn from_iter<Iter: IntoIterator<Item = (I, T)>>(iter: Iter) -> Self {
        let mut store = VecMap::new();
        for (index, value) in iter {
            store.insert(index, value);
        }
        store
    }
}

pub struct VecMapKeysIter<'a, I: VecMapIndex, T> {
    iter: std::iter::Enumerate<std::slice::Iter<'a, Option<T>>>,
    _phantom: PhantomData<I>,
}
impl<'a, I: VecMapIndex, T> VecMapKeysIter<'a, I, T> {
    pub fn new(iter: std::iter::Enumerate<std::slice::Iter<'a, Option<T>>>) -> Self {
        Self {
            iter,
            _phantom: PhantomData::default(),
        }
    }
}
impl<'a, I: VecMapIndex, T> Iterator for VecMapKeysIter<'a, I, T> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(el) = self.iter.next() {
            if let (i, Some(_)) = el {
                return Some(I::new_index(i));
            }
        }

        None
    }
}

pub struct VecMapEntriesIter<'a, I: VecMapIndex, T> {
    iter: std::iter::Enumerate<std::slice::Iter<'a, Option<T>>>,
    _phantom: PhantomData<I>,
}
impl<'a, I: VecMapIndex, T> VecMapEntriesIter<'a, I, T> {
    pub fn new(iter: std::iter::Enumerate<std::slice::Iter<'a, Option<T>>>) -> Self {
        Self {
            iter,
            _phantom: PhantomData::default(),
        }
    }
}
impl<'a, I: VecMapIndex, T> Iterator for VecMapEntriesIter<'a, I, T> {
    type Item = (I, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(el) = self.iter.next() {
            if let (i, Some(el)) = el {
                return Some((I::new_index(i), el));
            }
        }

        None
    }
}

pub struct VecMapEntriesIntoIter<I: VecMapIndex, T> {
    iter: std::iter::Enumerate<std::vec::IntoIter<Option<T>>>,
    _phantom: PhantomData<I>,
}
impl<I: VecMapIndex, T> VecMapEntriesIntoIter<I, T> {
    pub fn new(iter: std::iter::Enumerate<std::vec::IntoIter<Option<T>>>) -> Self {
        Self {
            iter,
            _phantom: PhantomData::default(),
        }
    }
}
impl<I: VecMapIndex, T> Iterator for VecMapEntriesIntoIter<I, T> {
    type Item = (I, T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(el) = self.iter.next() {
            if let (i, Some(el)) = el {
                return Some((I::new_index(i), el));
            }
        }

        None
    }
}
