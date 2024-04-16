use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Trait to obtain BTreeMap keys from elements that should be deserialised from a Vec.
pub trait BTreeVector<K> {
    fn key(&self) -> K;
}

/// Deserialises a [`Vec<V>`] into a [`BTreeMap<K, V>`]
pub fn deserialize_btree_vector<
    'de,
    D: Deserializer<'de>,
    K: Ord,
    V: Deserialize<'de> + BTreeVector<K>,
>(
    deserializer: D,
) -> Result<BTreeMap<K, V>, D::Error> {
    let vector = Vec::<V>::deserialize(deserializer)?;

    let mut ret = BTreeMap::new();
    for element in vector {
        let key = element.key();
        ret.insert(key, element);
    }

    Ok(ret)
}

/// Serialises a [`BTreeMap<K, V>`] as a sequence of its values.
pub fn serialise_btreemap<S: Serializer, K, V: Serialize>(
    map: &BTreeMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(map.values())
}

/// Serialises a [`BTreeMap<K, V>`] into a stable-sorted sequence of its values.
pub fn serialise_btreemap_and_sort<S: Serializer, K, V: Serialize + Ord>(
    map: &BTreeMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut values = map.values().collect::<Vec<&V>>();
    values.sort();

    serializer.collect_seq(values)
}
