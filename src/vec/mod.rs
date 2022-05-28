use std::ops::Add;

use num_traits::{Float, Num};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<T, Output = T>,
    T: Clone,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x.clone() + other.x.clone(),
            y: self.y.clone() + other.y.clone(),
            z: self.z.clone() + other.z.clone(),
        }
    }
}

impl<T> Vec3<T>
where
    T: Num,
    T: Clone,
{
    pub fn dot(&self, other: Self) -> T {
        self.x.clone() * other.x.clone()
            + self.y.clone() * other.y.clone()
            + self.z.clone() * other.z.clone()
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y.clone() * other.z.clone() - self.z.clone() * other.y.clone(),
            y: self.z.clone() * other.x.clone() - self.x.clone() * other.z.clone(),
            z: self.x.clone() * other.y.clone() - self.y.clone() * other.x.clone(),
        }
    }

    pub fn sq_length(&self) -> T {
        self.x.clone() * self.x.clone()
            + self.y.clone() * self.y.clone()
            + self.z.clone() * self.z.clone()
    }
}

impl<T> Vec3<T>
where
    T: Float,
{
    pub fn length(&self) -> T {
        self.sq_length().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x.clone() / len,
            y: self.y.clone() / len,
            z: self.z.clone() / len,
        }
    }

    pub fn cos(&self, other: Self) -> T {
        self.dot(other) / (self.length() * other.length())
    }

    pub fn angle(&self, other: Self) -> T {
        self.cos(other).acos()
    }
}
