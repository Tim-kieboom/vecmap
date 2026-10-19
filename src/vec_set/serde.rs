use serde::ser::SerializeTuple;

use crate::{vec_map_index::VecMapIndex, vec_set::VecSet};

impl<I: VecMapIndex> FromIterator<I> for VecSet<I> {
    fn from_iter<Iter: IntoIterator<Item = I>>(iter: Iter) -> Self {
        let mut set = Self::new();
        for i in iter {
            set.insert(i);
        }
        set
    }
}
impl<I> serde::Serialize for VecSet<I>
where
    I: serde::Serialize + VecMapIndex,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_tuple(self.len())?;
        for index in self.entries() {
            map.serialize_element(&index)?;
        }
        map.end()
    }
}

impl<'de, I> serde::Deserialize<'de> for VecSet<I>
where
    I: serde::Deserialize<'de> + VecMapIndex,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let list = Vec::<I>::deserialize(deserializer)?;
        Ok(VecSet::from_vec(list))
    }
}
