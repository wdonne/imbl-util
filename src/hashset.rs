//! Wrappers around `HashSet` to make it more composable.
use imbl::HashSet;
use std::hash::Hash;

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
