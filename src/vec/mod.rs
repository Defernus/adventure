use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use cgmath::{Point3, Vector3};
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

impl<T> Into<Vec3<T>> for (T, T, T) {
    fn into(self) -> Vec3<T> {
        Vec3::new(self.0, self.1, self.2)
    }
}

impl<T> From<Vec3<T>> for Vector3<T> {
    fn from(v: Vec3<T>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl<T> From<Vec3<T>> for Point3<T> {
    fn from(v: Vec3<T>) -> Self {
        Self::new(v.x, v.y, v.z)
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
impl<T> AddAssign for Vec3<T>
where
    T: AddAssign<T>,
    T: Clone,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
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

    // formula from https://en.wikipedia.org/wiki/Rodrigues%27_rotation_formula
    pub fn rotate(&self, axis: Self, a: T) -> Self {
        let cos = a.cos();
        let sin = a.sin();

        let a = self.clone() * cos;
        let b = axis.cross(self.clone()) * sin;
        let c = axis * axis.clone().dot(self.clone());
        let d: T = T::from::<f32>(1.).unwrap() - cos;
        a + b + c * d
    }
}
