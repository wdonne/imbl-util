//! Wrappers around `Vector` to make it more composable.
use imbl::Vector;
use std::cmp::Ordering;

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
