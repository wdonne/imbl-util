//! Wrappers around `OrdSet` to make it more composable.
use imbl::OrdSet;

pub trait OrdSetExt<T>
where
    T: Clone + Ord,
{
    /**
    ```rust
    # use imbl::{ordset, OrdSet};
    # use imbl_util::ordset::OrdSetExt;
    # fn main() {
    assert_eq!((ordset![0], None), OrdSet::new().insert_self(0))
    # }
    ```
    */
    fn insert_self(&self, value: T) -> (OrdSet<T>, Option<T>);

    /**
    ```rust
    # use imbl::{ordset, OrdSet};
    # use imbl_util::ordset::OrdSetExt;
    # fn main() {
    assert_eq!((ordset![0], Some(1)), ordset![0, 1].remove_self(&1))
    # }
    ```
    */
    fn remove_self(&self, value: &T) -> (OrdSet<T>, Option<T>);

    /**
    ```rust
    # use imbl::{ordset, OrdSet};
    # use imbl_util::ordset::OrdSetExt;
    # fn main() {
    assert_eq!((ordset![0], Some(1)), ordset![0, 1].remove_max_self())
    # }
    ```
    */
    fn remove_max_self(&self) -> (OrdSet<T>, Option<T>);

    /**
    ```rust
    # use imbl::{ordset, OrdSet};
    # use imbl_util::ordset::OrdSetExt;
    # fn main() {
    assert_eq!((ordset![1], Some(0)), ordset![0, 1].remove_min_self())
    # }
    ```
    */
    fn remove_min_self(&self) -> (OrdSet<T>, Option<T>);
}

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

impl<T> OrdSetExt<T> for OrdSet<T>
where
    T: Clone + Ord,
{
    fn insert_self(&self, value: T) -> (OrdSet<T>, Option<T>) {
        crate::ordset::insert(self, value)
    }

    fn remove_self(&self, value: &T) -> (OrdSet<T>, Option<T>) {
        crate::ordset::remove(self, value)
    }

    fn remove_max_self(&self) -> (OrdSet<T>, Option<T>) {
        crate::ordset::remove_max(self)
    }

    fn remove_min_self(&self) -> (OrdSet<T>, Option<T>) {
        crate::ordset::remove_min(self)
    }
}
