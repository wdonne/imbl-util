//! Wrappers around `OrdMap` to make it more composable.
use imbl::OrdMap;

pub trait OrdMapExt<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    /**
    ```rust
    # use imbl::{ordmap, OrdMap};
    # use imbl_util::ordmap::OrdMapExt;
    # fn main() {
    assert_eq!((ordmap!{0 => 0}, None), OrdMap::new().insert_self(0, 0));
    assert_eq!((ordmap!{0 => 1}, Some(0)), ordmap!{0 => 0}.insert_self(0, 1))
    # }
    ```
    */
    fn insert_self(&self, k: K, v: V) -> (OrdMap<K, V>, Option<V>);

    /**
    ```rust
    # use imbl::{ordmap, OrdMap};
    # use imbl_util::ordmap::OrdMapExt;
    # fn main() {
    assert_eq!((ordmap!{0 => 0}, Some(1)), ordmap!{0 => 0, 1 => 1}.remove_self(&1))
    # }
    ```
    */
    fn remove_self(&self, k: &K) -> (OrdMap<K, V>, Option<V>);

    /**
    ```rust
    # use imbl::{ordmap, OrdMap};
    # use imbl_util::ordmap::OrdMapExt;
    # fn main() {
    assert_eq!((ordmap!{0 => 0}, Some((1, 1))), ordmap!{0 => 0, 1 => 1}.remove_with_key_self(&1))
    # }
    ```
    */
    fn remove_with_key_self(&self, k: &K) -> (OrdMap<K, V>, Option<(K, V)>);
}

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

impl<K, V> OrdMapExt<K, V> for OrdMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn insert_self(&self, k: K, v: V) -> (OrdMap<K, V>, Option<V>) {
        crate::ordmap::insert(self, k, v)
    }

    fn remove_self(&self, k: &K) -> (OrdMap<K, V>, Option<V>) {
        crate::ordmap::remove(self, k)
    }

    fn remove_with_key_self(&self, k: &K) -> (OrdMap<K, V>, Option<(K, V)>) {
        crate::ordmap::remove_with_key(self, k)
    }
}
