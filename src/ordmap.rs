//! Wrappers around `OrdMap` to make it more composable.
use imbl::OrdMap;

pub fn insert<K, V>(map: &OrdMap<K, V>, k: K, v: V) -> (OrdMap<K, V>, Option<V>)
where
    K: Clone + Ord,
    V: Clone,
{
    update_ordmap_value(map, |m| m.insert(k, v))
}

pub fn remove<K, V>(map: &OrdMap<K, V>, k: &K) -> (OrdMap<K, V>, Option<V>)
where
    K: Clone + Ord,
    V: Clone,
{
    update_ordmap_value(map, |m| m.remove(k))
}

pub fn remove_with_key<K, V>(map: &OrdMap<K, V>, k: &K) -> (OrdMap<K, V>, Option<(K, V)>)
where
    K: Clone + Ord,
    V: Clone,
{
    let (map, value) = remove(map, k);

    (map, value.map(|v| (k.clone(), v)))
}

fn update_ordmap_value<K, V, F>(map: &OrdMap<K, V>, f: F) -> (OrdMap<K, V>, Option<V>)
where
    F: FnOnce(&mut OrdMap<K, V>) -> Option<V>,
    K: Clone,
    V: Clone,
{
    let mut new_map = map.clone();
    let value = f(&mut new_map);

    (new_map, value)
}
