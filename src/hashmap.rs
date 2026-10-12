//! Wrappers around `HashMap` to make it more composable.
use imbl::HashMap;
use std::hash::Hash;

pub trait HashMapExt<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    /**
    ```rust
    # use imbl::{hashmap, HashMap};
    # use imbl_util::hashmap::HashMapExt;
    # fn main() {
    assert_eq!((hashmap!{0 => 0}, None), HashMap::new().insert_self(0, 0));
    assert_eq!((hashmap!{0 => 1}, Some(0)), hashmap!{0 => 0}.insert_self(0, 1))
    # }
    ```
    */
    fn insert_self(&self, k: K, v: V) -> (HashMap<K, V>, Option<V>);

    /**
    ```rust
    # use imbl::{hashmap, HashMap};
    # use imbl_util::hashmap::HashMapExt;
    # fn main() {
    assert_eq!(hashmap!{0 => 0, 1 => 1}, hashmap!{0 => 0}.merge_self(&hashmap!{1 => 1}))
    # }
    ```
    */
    fn merge_self(&self, into: &HashMap<K, V>) -> HashMap<K, V>;

    /**
    ```rust
    # use imbl::{hashmap, HashMap};
    # use imbl_util::hashmap::HashMapExt;
    # fn main() {
    assert_eq!((hashmap!{0 => 0}, Some(1)), hashmap!{0 => 0, 1 => 1}.remove_self(&1))
    # }
    ```
    */
    fn remove_self(&self, k: &K) -> (HashMap<K, V>, Option<V>);

    /**
    ```rust
    # use imbl::{hashmap, HashMap};
    # use imbl_util::hashmap::HashMapExt;
    # fn main() {
    assert_eq!((hashmap!{0 => 0}, Some((1,1))), hashmap!{0 => 0, 1 => 1}.remove_with_key_self(&1))
    # }
    ```
    */
    fn remove_with_key_self(&self, k: &K) -> (HashMap<K, V>, Option<(K, V)>);

    /**
    ```rust
    # use imbl::{hashmap, HashMap};
    # use imbl_util::hashmap::HashMapExt;
    # fn main() {
    assert_eq!(hashmap!{0 => 0}, hashmap!{0 => 0, 1 => 1}.retain_self(|k, v| *k == 0 && *v == 0))
    # }
    ```
    */
    fn retain_self<F>(&self, f: F) -> HashMap<K, V>
    where
        F: FnMut(&K, &V) -> bool;
}

pub fn insert<K, V>(map: &HashMap<K, V>, k: K, v: V) -> (HashMap<K, V>, Option<V>)
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    update_hashmap_value(map, |m| m.insert(k, v))
}

pub fn merge<K, V>(map: &HashMap<K, V>, into: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    map.clone().union(into.clone())
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

impl<K, V> HashMapExt<K, V> for HashMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn insert_self(&self, k: K, v: V) -> (HashMap<K, V>, Option<V>) {
        crate::hashmap::insert(self, k, v)
    }

    fn merge_self(&self, into: &HashMap<K, V>) -> HashMap<K, V> {
        crate::hashmap::merge(self, into)
    }

    fn remove_self(&self, k: &K) -> (HashMap<K, V>, Option<V>) {
        crate::hashmap::remove(self, k)
    }

    fn remove_with_key_self(&self, k: &K) -> (HashMap<K, V>, Option<(K, V)>) {
        crate::hashmap::remove_with_key(self, k)
    }

    fn retain_self<F>(&self, f: F) -> HashMap<K, V>
    where
        F: FnMut(&K, &V) -> bool,
    {
        crate::hashmap::retain(self, f)
    }
}
