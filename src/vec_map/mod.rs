use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub mod entry;
pub mod iter;
pub mod serde;

use crate::ids::IdAlloc;
use crate::vec_map_index::VecMapIndex;

/// A sparse map-like data structure backed by a vector.
///
/// Each index `I` maps to an optional value `T`. This allows efficient indexing
/// by numeric-like types while maintaining the flexibility of a `HashMap`-like interface.
///
/// Internally, the structure stores a `Vec<Option<T>>`, expanding as needed.
#[derive(Debug, Clone)]
pub struct VecMap<I: VecMapIndex, T> {
    pub vec: Vec<Option<T>>,
    _marker: PhantomData<I>,
}
impl<I: VecMapIndex, T> VecMap<I, T> {
    pub const fn new() -> Self {
        Self::const_default()
    }

    pub const fn const_default() -> Self {
        Self {
            vec: vec![],
            _marker: PhantomData,
        }
    }

    /// Returns the current length of the internal vector (including `None` entries).
    pub const fn cap(&self) -> usize {
        self.vec.len()
    }

    /// returns amount of entries
    pub fn len(&self) -> usize {
        self.vec.iter().filter(|o| o.is_some()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Creates a new [`VecMap`] with a given capacity of `cap`.
    ///
    /// The internal vector will contain `cap` empty (`None`) entries.
    pub fn with_capacity(cap: usize) -> Self {
        let mut vec = Vec::with_capacity(cap);
        vec.resize_with(cap, || None);

        Self {
            vec,
            _marker: PhantomData,
        }
    }

    /// Inserts or replaces a value at the specified index.
    ///
    /// If the index exceeds the current vector length, the internal vector
    /// automatically grows to accommodate it.
    pub fn insert(&mut self, index: I, value: T) -> Option<T> {
        let index = index.index();
        self.inner_insert(index, value)
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    /// Creates a [`VecMap`] from a vector of index-value pairs.
    pub fn from_vec(entries: Vec<(I, T)>) -> Self {
        let max_index = entries.iter().map(|(i, _)| i.index()).max().unwrap_or(0);

        let mut this = Self::with_capacity(max_index);
        for (index, value) in entries {
            this.insert(index, value);
        }
        this
    }

    /// Checks whether the given index `I` currently contains a value.
    pub fn contains(&self, index: I) -> bool {
        self.vec.get(index.index()).is_some_and(|el| el.is_some())
    }

    /// Returns a reference to the value at the given index, if present.
    pub fn get(&self, index: I) -> Option<&T> {
        match self.vec.get(index.index()) {
            Some(Some(val)) => Some(val),
            _ => None,
        }
    }

    /// Returns a reference to the value at the given index, if present.
    pub fn get_or_error_id(&self, index: I) -> T
    where
        T: IdAlloc + Copy,
    {
        match self.vec.get(index.index()) {
            Some(Some(val)) => *val,
            _ => T::error(),
        }
    }

    /// Returns a mutable reference to the value at the given index, if present.
    pub fn get_or_default(&mut self, index: I) -> &T
    where
        T: Default,
    {
        match self.vec.get(index.index()) {
            Some(None) | None => _ = self.inner_insert(index.index(), Default::default()),
            _ => (),
        }

        self.vec[index.index()].as_ref().expect("on None")
    }

    /// Returns a mutable reference to the value at the given index, if present.
    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        match self.vec.get_mut(index.index()) {
            Some(Some(val)) => Some(val),
            _ => None,
        }
    }

    /// Returns a mutable reference to the value at the given index, if present.
    pub fn get_mut_or_default(&mut self, index: I) -> &mut T
    where
        T: Default,
    {
        match self.vec.get(index.index()) {
            Some(None) | None => _ = self.inner_insert(index.index(), Default::default()),
            _ => (),
        }

        self.vec[index.index()]
            .as_mut()
            .expect("entry in VecMap::get_mut_or_default should be Some(_)")
    }

    /// Raw index of inner vec
    pub fn raw_index(&self, index: usize) -> Option<&T> {
        match self.vec.get(index) {
            Some(val) => val.as_ref(),
            None => None,
        }
    }

    /// Extends this map by inserting multiple `(index, value)` pairs from an iterator.
    pub fn extend<Iter>(&mut self, vec: Iter)
    where
        Iter: Iterator<Item = (I, T)>,
    {
        for (index, value) in vec {
            self.insert(index, value);
        }
    }

    /// Removes and returns the value at the specified index, if present.
    pub fn remove(&mut self, index: I) -> Option<T> {
        self.vec.get_mut(index.index()).and_then(|slot| slot.take())
    }

    fn inner_insert(&mut self, index: usize, value: T) -> Option<T> {
        if index >= self.vec.len() {
            self.vec.resize_with(index + 1, || None);
        }
        self.vec[index].replace(value)
    }
}

impl<I: VecMapIndex, T: Default> Default for VecMap<I, T> {
    fn default() -> Self {
        Self::const_default()
    }
}

impl<I: VecMapIndex + Debug, T> Index<I> for VecMap<I, T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.vec[index.index()]
            .as_ref()
            .unwrap_or_else(|| panic!("entry in VecMap[{:?}] not found", index))
    }
}

impl<I: VecMapIndex + Debug, T> IndexMut<I> for VecMap<I, T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.vec[index.index()]
            .as_mut()
            .unwrap_or_else(|| panic!("entry in VecMap[{:?}] not found", index))
    }
}

impl<I: VecMapIndex + Clone, T: Clone> VecMap<I, T> {
    /// Constructs a [`VecMap`] from a slice of index-value pairs.
    ///
    /// Each `(I, T)` pair is inserted into the map, resizing the underlying vector as needed.
    pub fn from_slice(slice: &[(I, T)]) -> Self {
        let mut this = Self::new();
        for (index, value) in slice.iter().cloned() {
            this.insert(index, value);
        }
        this
    }
}

impl<I: VecMapIndex + Eq, T: Eq> Eq for VecMap<I, T> {}
impl<I: VecMapIndex + PartialEq, T: PartialEq> PartialEq for VecMap<I, T> {
    fn eq(&self, other: &Self) -> bool {
        self.entries().eq(other.entries())
    }
}
