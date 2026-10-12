//! Wrappers around `HashSet` to make it more composable.
use imbl::HashSet;
use std::hash::Hash;

pub trait HashSetExt<T>
where
    T: Clone + Hash + Eq,
{
    /**
    ```rust
    # use imbl::{hashset, HashSet};
    # use imbl_util::hashset::HashSetExt;
    # fn main() {
    assert_eq!((hashset![0], None), HashSet::new().insert_self(0))
    # }
    ```
    */
    fn insert_self(&self, value: T) -> (HashSet<T>, Option<T>);

    /**
    ```rust
    # use imbl::{hashset, HashSet};
    # use imbl_util::hashset::HashSetExt;
    # fn main() {
    assert_eq!((hashset![0], Some(1)), hashset![0, 1].remove_self(&1))
    # }
    ```
    */
    fn remove_self(&self, value: &T) -> (HashSet<T>, Option<T>);

    /**
    ```rust
    # use imbl::{hashset, HashSet};
    # use imbl_util::hashset::HashSetExt;
    # fn main() {
    assert_eq!(hashset![0], hashset![0, 1].retain_self(|v| *v == 0))
    # }
    ```
    */
    fn retain_self<F>(&self, f: F) -> HashSet<T>
    where
        F: FnMut(&T) -> bool;
}

pub fn insert<T>(map: &HashSet<T>, value: T) -> (HashSet<T>, Option<T>)
where
    T: Clone + Hash + Eq,
{
    update_hashset_value(map, |m| m.insert(value))
}

pub fn remove<T>(map: &HashSet<T>, value: &T) -> (HashSet<T>, Option<T>)
where
    T: Clone + Hash + Eq,
{
    update_hashset_value(map, |m| m.remove(value))
}

pub fn retain<T, F>(map: &HashSet<T>, f: F) -> HashSet<T>
where
    F: FnMut(&T) -> bool,
    T: Clone + Hash + Eq,
{
    update_hashset(map, |m| m.retain(f))
}

fn update_hashset<T, F>(map: &HashSet<T>, f: F) -> HashSet<T>
where
    F: FnOnce(&mut HashSet<T>),
    T: Clone,
{
    let mut new_map = map.clone();

    f(&mut new_map);
    new_map
}

fn update_hashset_value<T, F>(map: &HashSet<T>, f: F) -> (HashSet<T>, Option<T>)
where
    F: FnOnce(&mut HashSet<T>) -> Option<T>,
    T: Clone,
{
    let mut new_map = map.clone();
    let value = f(&mut new_map);

    (new_map, value)
}

impl<T> HashSetExt<T> for HashSet<T>
where
    T: Clone + Hash + Eq,
{
    fn insert_self(&self, value: T) -> (HashSet<T>, Option<T>) {
        crate::hashset::insert(self, value)
    }

    fn remove_self(&self, value: &T) -> (HashSet<T>, Option<T>) {
        crate::hashset::remove(self, value)
    }

    fn retain_self<F>(&self, f: F) -> HashSet<T>
    where
        F: FnMut(&T) -> bool,
    {
        crate::hashset::retain(self, f)
    }
}
