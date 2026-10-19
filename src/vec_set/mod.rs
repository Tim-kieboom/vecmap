use std::ops::Index;

use crate::{
    vec_map::{VecMap, iter::VecMapKeysIter},
    vec_map_index::VecMapIndex,
};

pub mod serde;

/// A set-like data structure backed by a [`VecMap`].
///
/// Each unique index `I` maps to a presence marker `()`.
/// This provides O(1) lookup and set operations using ID types.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VecSet<I: VecMapIndex> {
    map: VecMap<I, ()>,
}
impl<I: VecMapIndex> VecSet<I> {
    /// Creates a new empty [`VecSet`].
    pub const fn new() -> Self {
        Self {
            map: VecMap::const_default(),
        }
    }

    /// Creates a new [`VecSet`] with the given capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            map: VecMap::with_capacity(cap),
        }
    }

    /// Creates a [`VecSet`] from a [`VecMap`] with `()` values.
    pub fn from_vecmap(map: VecMap<I, ()>) -> Self {
        Self { map }
    }

    /// Creates a [`VecSet`] from a vector of indices.
    pub fn from_vec(keys: Vec<I>) -> Self {
        let max_index = keys.iter().map(|i| i.index()).max().unwrap_or(0);

        let mut this = Self::with_capacity(max_index);
        for index in keys {
            this.insert(index);
        }
        this
    }

    /// Creates a [`VecSet`] from a slice of indices.
    pub fn from_slice(keys: &[I]) -> Self {
        let max_index = keys.iter().map(|i| i.index()).max().unwrap_or(0);

        let mut this = Self::with_capacity(max_index);
        for key in keys {
            let index = I::new_index(key.index());
            this.insert(index);
        }
        this
    }

    /// Inserts an index into the set.
    ///
    /// Returns the previous value at that index, or `None` if it was empty.
    pub fn insert(&mut self, index: I) -> Option<()> {
        self.map.insert(index, ())
    }

    /// Returns the number of indices in the set.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Checks whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Checks whether the set contains the given index.
    pub fn contains(&self, index: I) -> bool {
        self.map.contains(index)
    }

    /// Removes and returns the index from the set, if present.
    pub fn remove(&mut self, index: I) -> Option<()> {
        self.map.remove(index)
    }

    /// Returns an iterator over all indices in the set.
    pub fn entries(&self) -> VecMapKeysIter<'_, I, ()> {
        self.iter()
    }

    /// Returns an iterator over all indices in the set.
    pub fn iter(&self) -> VecMapKeysIter<'_, I, ()> {
        self.map.keys()
    }

    /// Extends this set by inserting multiple indices from an iterator.
    pub fn extend<Iter>(&mut self, iter: Iter)
    where
        Iter: Iterator<Item = I>,
    {
        for index in iter {
            self.insert(index);
        }
    }

    /// Returns an iterator over indices present in both `self` and `other`.
    pub fn intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = I> + 'a {
        self.iter()
            .filter(move |index| other.contains(I::new_index(index.index())))
    }
}
impl<'a, I: VecMapIndex> IntoIterator for &'a VecSet<I> {
    type Item = I;
    type IntoIter = VecMapKeysIter<'a, I, ()>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries()
    }
}

impl<I: VecMapIndex> Index<I> for VecSet<I> {
    type Output = bool;

    fn index(&self, index: I) -> &Self::Output {
        if self.map.contains(index) {
            &true
        } else {
            &false
        }
    }
}
impl<I: VecMapIndex, const N: usize> From<[I; N]> for VecSet<I> {
    fn from(value: [I; N]) -> Self {
        VecSet::from_slice(&value)
    }
}
