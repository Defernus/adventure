use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl<T> Sub for Vec3<T>
where
    T: Sub<T, Output = T>,
    T: Clone,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x.clone() - other.x.clone(),
            y: self.y.clone() - other.y.clone(),
            z: self.z.clone() - other.z.clone(),
        }
    }
}

impl<T> Mul<Vec3<T>> for Vec3<T>
where
    T: Mul<T, Output = T>,
    T: Clone,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x.clone() * other.x.clone(),
            y: self.y.clone() * other.y.clone(),
            z: self.z.clone() * other.z.clone(),
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<T, Output = T>,
    T: Clone,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x.clone() * other.clone(),
            y: self.y.clone() * other.clone(),
            z: self.z.clone() * other.clone(),
        }
    }
}

impl<T> Div<Vec3<T>> for Vec3<T>
where
    T: Div<T, Output = T>,
    T: Clone,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x.clone() / other.x.clone(),
            y: self.y.clone() / other.y.clone(),
            z: self.z.clone() / other.z.clone(),
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<T, Output = T>,
    T: Clone,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x.clone() / other.clone(),
            y: self.y.clone() / other.clone(),
            z: self.z.clone() / other.clone(),
        }
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T>,
    T: Clone,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x.clone(),
            y: -self.y.clone(),
            z: -self.z.clone(),
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
        self.clone() / len
    }

    pub fn cos(&self, other: Self) -> T {
        self.dot(other) / (self.length() * other.length())
    }

    pub fn angle(&self, other: Self) -> T {
        self.cos(other).acos()
    }
}
