//! Wrappers around `Vector` to make it more composable.
use imbl::Vector;
use std::cmp::Ordering;

pub trait VectorExt<T> {
    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!(vector![0, 1], vector![0].append_self(&vector![1]))
    # }
    ```
    */
    fn append_self(&self, vector: &Vector<T>) -> Vector<T>;

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!(Some(vector![0]), Vector::new().insert_self(0, 0))
    # }
    ```
    */
    fn insert_self(&self, index: usize, value: T) -> Option<Vector<T>>;

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!((vector![0], Some(1)), vector![0, 1].pop_back_self())
    # }
    ```
    */
    fn pop_back_self(&self) -> (Vector<T>, Option<T>);

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!((vector![1], Some(0)), vector![0, 1].pop_front_self())
    # }
    ```
    */
    fn pop_front_self(&self) -> (Vector<T>, Option<T>);

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!(vector![0, 1], vector![0].push_back_self(1))
    # }
    ```
    */
    fn push_back_self(&self, value: T) -> Vector<T>;

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!(vector![0, 1], vector![1].push_front_self(0))
    # }
    ```
    */
    fn push_front_self(&self, value: T) -> Vector<T>;

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!((vector![0], Some(1)), vector![0, 1].remove_self(1))
    # }
    ```
    */
    fn remove_self(&self, index: usize) -> (Vector<T>, Option<T>);

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!(vector![0], vector![0, 1].retain_self(|v| v < &1))
    # }
    ```
    */
    fn retain_self<F>(&self, f: F) -> Vector<T>
    where
        F: FnMut(&T) -> bool;

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!((vector![0, 2], Some(1)), vector![0, 1].set_self(1, 2))
    # }
    ```
    */
    fn set_self(&self, index: usize, value: T) -> (Vector<T>, Option<T>);

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!(vector![1, 0], vector![0, 1].swap_self(0, 1))
    # }
    ```
    */
    fn swap_self(&self, i: usize, j: usize) -> Vector<T>;

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExt;
    # fn main() {
    assert_eq!(vector![0], vector![0, 1].truncate_self(1))
    # }
    ```
    */
    fn truncate_self(&self, len: usize) -> Vector<T>;
}

pub trait VectorExtOrd<T> {
    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExtOrd;
    # fn main() {
    assert_eq!(vector![0], Vector::new().insert_ord_self(0))
    # }
    ```
    */
    fn insert_ord_self(&self, value: T) -> Vector<T>;

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExtOrd;
    # fn main() {
    assert_eq!(vector![0, 1], vector![1].insert_ord_by_self(0, i32::cmp))
    # }
    ```
    */
    fn insert_ord_by_self<F>(&self, value: T, f: F) -> Vector<T>
    where
        F: FnMut(&T, &T) -> Ordering;

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExtOrd;
    # fn main() {
    assert_eq!(vector![0, 1], vector![1].insert_ord_by_key_self(0, |v| *v))
    # }
    ```
    */
    fn insert_ord_by_key_self<F, K>(&self, value: T, f: F) -> Vector<T>
    where
        F: FnMut(&T) -> K,
        K: Ord;

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExtOrd;
    # fn main() {
    assert_eq!(vector![0, 1], vector![1, 0].sort_self())
    # }
    ```
    */
    fn sort_self(&self) -> Vector<T>;

    /**
    ```rust
    # use imbl::{vector, Vector};
    # use imbl_util::vector::VectorExtOrd;
    # fn main() {
    assert_eq!(vector![0, 1], vector![1, 0].sort_by_self(i32::cmp))
    # }
    ```
    */
    fn sort_by_self<F>(&self, f: F) -> Vector<T>
    where
        F: Fn(&T, &T) -> Ordering;
}

pub fn append<T>(vector: &Vector<T>, other: &Vector<T>) -> Vector<T>
where
    T: Clone,
{
    update_vector(vector, |v| v.append(other.clone()))
}

pub fn insert<T>(vector: &Vector<T>, index: usize, value: T) -> Option<Vector<T>>
where
    T: Clone,
{
    if index <= vector.len() {
        Some(update_vector(vector, |v| v.insert(index, value)))
    } else {
        None
    }
}

pub fn insert_ord<T>(vector: &Vector<T>, value: T) -> Vector<T>
where
    T: Clone + Ord,
{
    update_vector(vector, |v| v.insert_ord(value))
}

pub fn insert_ord_by<T, F>(vector: &Vector<T>, value: T, f: F) -> Vector<T>
where
    F: FnMut(&T, &T) -> Ordering,
    T: Clone + Ord,
{
    update_vector(vector, |v| v.insert_ord_by(value, f))
}

pub fn insert_ord_by_key<T, F, K>(vector: &Vector<T>, value: T, f: F) -> Vector<T>
where
    F: FnMut(&T) -> K,
    K: Ord,
    T: Clone,
{
    update_vector(vector, |v| v.insert_ord_by_key(value, f))
}

pub fn pop_back<T>(vector: &Vector<T>) -> (Vector<T>, Option<T>)
where
    T: Clone,
{
    update_vector_value(vector, |v| v.pop_back())
}

pub fn pop_front<T>(vector: &Vector<T>) -> (Vector<T>, Option<T>)
where
    T: Clone,
{
    update_vector_value(vector, |v| v.pop_front())
}

pub fn push_back<T>(vector: &Vector<T>, value: T) -> Vector<T>
where
    T: Clone,
{
    update_vector(vector, |v| v.push_back(value))
}

pub fn push_front<T>(vector: &Vector<T>, value: T) -> Vector<T>
where
    T: Clone,
{
    update_vector(vector, |v| v.push_front(value))
}

pub fn remove<T>(vector: &Vector<T>, index: usize) -> (Vector<T>, Option<T>)
where
    T: Clone,
{
    if index < vector.len() {
        update_vector_value(vector, |v| Some(v.remove(index)))
    } else {
        (vector.clone(), None)
    }
}

pub fn retain<T, F>(vector: &Vector<T>, f: F) -> Vector<T>
where
    F: FnMut(&T) -> bool,
    T: Clone,
{
    update_vector(vector, |v| v.retain(f))
}

pub fn set<T>(vector: &Vector<T>, index: usize, value: T) -> (Vector<T>, Option<T>)
where
    T: Clone,
{
    if index < vector.len() {
        update_vector_value(vector, |v| Some(v.set(index, value)))
    } else {
        (vector.clone(), None)
    }
}

pub fn sort<T>(vector: &Vector<T>) -> Vector<T>
where
    T: Clone + Ord,
{
    update_vector(vector, |v| v.sort())
}

pub fn sort_by<T, F>(vector: &Vector<T>, f: F) -> Vector<T>
where
    F: Fn(&T, &T) -> Ordering,
    T: Clone + Ord,
{
    update_vector(vector, |v| v.sort_by(f))
}

pub fn swap<T>(vector: &Vector<T>, i: usize, j: usize) -> Vector<T>
where
    T: Clone,
{
    update_vector(vector, |v| v.swap(i, j))
}

pub fn truncate<T>(vector: &Vector<T>, len: usize) -> Vector<T>
where
    T: Clone,
{
    update_vector(vector, |v| v.truncate(len))
}

fn update_vector<T, F>(vector: &Vector<T>, f: F) -> Vector<T>
where
    F: FnOnce(&mut Vector<T>),
    T: Clone,
{
    let mut new_vec = vector.clone();

    f(&mut new_vec);
    new_vec
}

fn update_vector_value<T, F>(vector: &Vector<T>, f: F) -> (Vector<T>, Option<T>)
where
    F: FnOnce(&mut Vector<T>) -> Option<T>,
    T: Clone,
{
    let mut new_vec = vector.clone();
    let value = f(&mut new_vec);

    (new_vec, value)
}

impl<T> VectorExt<T> for Vector<T>
where
    T: Clone,
{
    fn append_self(&self, vector: &Vector<T>) -> Vector<T> {
        crate::vector::append(self, vector)
    }

    fn insert_self(&self, index: usize, value: T) -> Option<Vector<T>> {
        crate::vector::insert(self, index, value)
    }

    fn pop_back_self(&self) -> (Vector<T>, Option<T>) {
        crate::vector::pop_back(self)
    }

    fn pop_front_self(&self) -> (Vector<T>, Option<T>) {
        crate::vector::pop_front(self)
    }

    fn push_back_self(&self, value: T) -> Vector<T> {
        crate::vector::push_back(self, value)
    }

    fn push_front_self(&self, value: T) -> Vector<T> {
        crate::vector::push_front(self, value)
    }

    fn remove_self(&self, index: usize) -> (Vector<T>, Option<T>) {
        crate::vector::remove(self, index)
    }

    fn retain_self<F>(&self, f: F) -> Vector<T>
    where
        F: FnMut(&T) -> bool,
    {
        crate::vector::retain(self, f)
    }

    fn set_self(&self, index: usize, value: T) -> (Vector<T>, Option<T>) {
        crate::vector::set(self, index, value)
    }

    fn swap_self(&self, i: usize, j: usize) -> Vector<T> {
        crate::vector::swap(self, i, j)
    }

    fn truncate_self(&self, len: usize) -> Vector<T> {
        crate::vector::truncate(self, len)
    }
}

impl<T> VectorExtOrd<T> for Vector<T>
where
    T: Clone + Ord,
{
    fn insert_ord_by_key_self<F, K>(&self, value: T, f: F) -> Vector<T>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        crate::vector::insert_ord_by_key(self, value, f)
    }

    fn insert_ord_by_self<F>(&self, value: T, f: F) -> Vector<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        crate::vector::insert_ord_by(self, value, f)
    }

    fn insert_ord_self(&self, value: T) -> Vector<T> {
        crate::vector::insert_ord(self, value)
    }

    fn sort_self(&self) -> Vector<T> {
        crate::vector::sort(self)
    }

    fn sort_by_self<F>(&self, f: F) -> Vector<T>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        crate::vector::sort_by(self, f)
    }
}
