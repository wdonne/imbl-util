//! Wrappers around `HashMap` to make it more composable.
use imbl::HashMap;
use std::hash::Hash;

pub fn merge<K, V>(map: &HashMap<K, V>, into: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    map.clone().union(into.clone())
}

pub fn insert<K, V>(map: &HashMap<K, V>, k: K, v: V) -> (HashMap<K, V>, Option<V>)
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    update_hashmap_value(map, |m| m.insert(k, v))
}

pub fn remove<K, V>(map: &HashMap<K, V>, k: &K) -> (HashMap<K, V>, Option<V>)
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    update_hashmap_value(map, |m| m.remove(k))
}

pub fn remove_with_key<K, V>(map: &HashMap<K, V>, k: &K) -> (HashMap<K, V>, Option<(K, V)>)
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    let (map, value) = remove(map, k);

    (map, value.map(|v| (k.clone(), v)))
}

pub fn retain<K, V, F>(map: &HashMap<K, V>, f: F) -> HashMap<K, V>
where
    F: FnMut(&K, &V) -> bool,
    K: Clone + Hash + Eq,
    V: Clone,
{
    update_hashmap(map, |m| m.retain(f))
}

fn update_hashmap<K, V, F>(map: &HashMap<K, V>, f: F) -> HashMap<K, V>
where
    F: FnOnce(&mut HashMap<K, V>),
    K: Clone,
    V: Clone,
{
    let mut new_map = map.clone();

    f(&mut new_map);
    new_map
}

fn update_hashmap_value<K, V, F>(map: &HashMap<K, V>, f: F) -> (HashMap<K, V>, Option<V>)
where
    F: FnOnce(&mut HashMap<K, V>) -> Option<V>,
    K: Clone,
    V: Clone,
{
    let mut new_map = map.clone();
    let value = f(&mut new_map);

    (new_map, value)
}
