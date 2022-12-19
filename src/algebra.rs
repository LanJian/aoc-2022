use anyhow::{anyhow, Error};
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point3<T = i64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T>
where
    T: Copy + From<i64> + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn cardinal_neighbours(&self) -> [Self; 6] {
        [
            Self::new(self.x + T::from(1), self.y, self.z),
            Self::new(self.x, self.y + T::from(1), self.z),
            Self::new(self.x, self.y, self.z + T::from(1)),
            Self::new(self.x - T::from(1), self.y, self.z),
            Self::new(self.x, self.y - T::from(1), self.z),
            Self::new(self.x, self.y, self.z - T::from(1)),
        ]
    }
}

impl<T> FromStr for Point3<T>
where
    T: FromStr,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(',').map(|t| t.trim().parse::<T>());

        Ok(Self {
            x: iter
                .next()
                .ok_or_else(|| anyhow!("Not enough values for Point3"))?
                .map_err(|_| anyhow!("Failed to parse value for Point3"))?,
            y: iter
                .next()
                .ok_or_else(|| anyhow!("Not enough values for Point3"))?
                .map_err(|_| anyhow!("Failed to parse value for Point3"))?,
            z: iter
                .next()
                .ok_or_else(|| anyhow!("Not enough values for Point3"))?
                .map_err(|_| anyhow!("Failed to parse value for Point3"))?,
        })
    }
}

impl<T> From<(T, T, T)> for Point3<T> {
    fn from(v: (T, T, T)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl<T> Add<Vector3<T>> for Point3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Point3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Sub<Vector3<T>> for Point3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Vector3<T = i64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Vector3<T> {
    fn from(v: (T, T, T)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}
