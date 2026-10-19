use std::marker::PhantomData;

use serde::{Serialize, Serializer, ser::SerializeMap};

use crate::{vec_map::VecMap, vec_map_index::VecMapIndex};

impl<I, T> serde::Serialize for VecMap<I, T>
where
    I: Serialize + VecMapIndex,
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.vec.len()))?;
        for (idx, value) in self.entries() {
            map.serialize_entry(&idx, value)?;
        }
        map.end()
    }
}

impl<'de, I, T> serde::Deserialize<'de> for VecMap<I, T>
where
    I: serde::Deserialize<'de> + VecMapIndex,
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(VecMapVisitor::new())
    }
}

struct VecMapVisitor<I: VecMapIndex, T> {
    marker: PhantomData<fn() -> VecMap<I, T>>,
}

impl<I: VecMapIndex, T> VecMapVisitor<I, T> {
    fn new() -> Self {
        VecMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, I, T> serde::de::Visitor<'de> for VecMapVisitor<I, T>
where
    I: serde::Deserialize<'de> + VecMapIndex,
    T: serde::Deserialize<'de>,
{
    type Value = VecMap<I, T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map with VecMap indices")
    }

    fn visit_map<M>(self, mut map: M) -> Result<VecMap<I, T>, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let mut vecmap: VecMap<I, T> = VecMap::new();

        while let Some((idx, value)) = map.next_entry::<I, T>()? {
            vecmap.insert(idx, value);
        }
        Ok(vecmap)
    }
}
