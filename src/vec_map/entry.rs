use crate::{vec_map::VecMap, vec_map_index::VecMapIndex};

impl<I: VecMapIndex, T> VecMap<I, T> {
    /// Returns the [`Entry`] for the given index, allowing in-place inspection,
    /// insertion, or modification of the corresponding slot.
    pub fn entry(&mut self, index: I) -> Entry<'_, T> {
        let index = index.index();
        if index >= self.vec.len() {
            self.vec.resize_with(index + 1, || None);
        }

        let slot = &mut self.vec[index];
        if slot.is_some() {
            Entry::Occupied(OccupiedEntry { slot })
        } else {
            Entry::Vacant(VacantEntry { slot })
        }
    }
}

/// A view into a single slot of a [`VecMap`], mirroring [`std::collections::hash_map::Entry`].
pub enum Entry<'a, T> {
    Occupied(OccupiedEntry<'a, T>),
    Vacant(VacantEntry<'a, T>),
}
impl<'a, T> Entry<'a, T> {
    /// Ensures a value is present by inserting `default` if the entry is vacant,
    /// then returns a mutable reference to the value.
    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is present by calling `default` if the entry is vacant,
    /// then returns a mutable reference to the value.
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Ensures a value is present by inserting `T::default()` if the entry is vacant,
    /// then returns a mutable reference to the value.
    pub fn or_default(self) -> &'a mut T
    where
        T: Default,
    {
        self.or_insert_with(Default::default)
    }

    /// Applies `f` to the value if the entry is occupied, then returns the entry unchanged.
    pub fn and_modify<F: FnOnce(&mut T)>(self, f: F) -> Self {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

/// An occupied [`Entry`], guaranteed to currently hold a value.
pub struct OccupiedEntry<'a, T> {
    slot: &'a mut Option<T>,
}
impl<'a, T> OccupiedEntry<'a, T> {
    /// Returns a reference to the occupied value.
    pub fn get(&self) -> &T {
        self.slot
            .as_ref()
            .expect("OccupiedEntry always holds a value")
    }

    /// Returns a mutable reference to the occupied value.
    pub fn get_mut(&mut self) -> &mut T {
        self.slot
            .as_mut()
            .expect("OccupiedEntry always holds a value")
    }

    /// Converts the entry into a mutable reference tied to the map's lifetime.
    pub fn into_mut(self) -> &'a mut T {
        self.slot
            .as_mut()
            .expect("OccupiedEntry always holds a value")
    }

    /// Replaces the occupied value, returning the previous one.
    pub fn insert(&mut self, value: T) -> T {
        self.slot
            .replace(value)
            .expect("OccupiedEntry always holds a value")
    }

    /// Removes and returns the occupied value, leaving the slot vacant.
    pub fn remove(self) -> T {
        self.slot
            .take()
            .expect("OccupiedEntry always holds a value")
    }
}

/// A vacant [`Entry`], guaranteed to currently hold no value.
pub struct VacantEntry<'a, T> {
    slot: &'a mut Option<T>,
}
impl<'a, T> VacantEntry<'a, T> {
    /// Inserts `value` into the vacant slot and returns a mutable reference to it.
    pub fn insert(self, value: T) -> &'a mut T {
        self.slot.insert(value)
    }
}
