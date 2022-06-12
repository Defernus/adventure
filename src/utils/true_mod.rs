use std::ops::{Add, Rem};

pub fn true_mod<T: Rem<T, Output = T> + Add<T, Output = T> + Clone>(a: T, b: T) -> T {
    (a.clone() % b.clone() + b.clone()) % b.clone()
}
