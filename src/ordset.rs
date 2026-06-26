//! Wrappers around `OrdSet` to make it more composable.
use imbl::OrdSet;

pub fn insert<T>(map: &OrdSet<T>, value: T) -> (OrdSet<T>, Option<T>)
where
    T: Clone + Ord,
{
    update_ordset_value(map, |m| m.insert(value))
}

pub fn remove<T>(map: &OrdSet<T>, value: &T) -> (OrdSet<T>, Option<T>)
where
    T: Clone + Ord,
{
    update_ordset_value(map, |m| m.remove(value))
}

pub fn remove_max<T>(map: &OrdSet<T>) -> (OrdSet<T>, Option<T>)
where
    T: Clone + Ord,
{
    update_ordset_value(map, |m| m.remove_max())
}

pub fn remove_min<T>(map: &OrdSet<T>) -> (OrdSet<T>, Option<T>)
where
    T: Clone + Ord,
{
    update_ordset_value(map, |m| m.remove_min())
}

fn update_ordset_value<T, F>(map: &OrdSet<T>, f: F) -> (OrdSet<T>, Option<T>)
where
    F: FnOnce(&mut OrdSet<T>) -> Option<T>,
    T: Clone,
{
    let mut new_map = map.clone();
    let value = f(&mut new_map);

    (new_map, value)
}
