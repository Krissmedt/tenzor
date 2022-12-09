use std::fmt::{Display, Formatter, write};
use std::ops;
use crate::math::IRealNumber;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: IRealNumber> ops::Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Vector3<T> {
        return Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl<T: IRealNumber> ops::Add<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: T) -> Vector3<T> {
        return Vector3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        };
    }
}


impl<T: IRealNumber> ops::Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
        return Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl<T: IRealNumber> ops::Sub<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: T) -> Vector3<T> {
        return Vector3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        };
    }
}

impl<T: IRealNumber> ops::Mul<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        return Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}

impl<T: IRealNumber> ops::Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl<T: IRealNumber> ops::Div<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: Vector3<T>) -> Self::Output {
        return Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        };
    }
}

impl<T: IRealNumber> ops::Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Self::Output {
        return Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

impl<T: IRealNumber> ops::Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        return Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl<T: IRealNumber> Vector3<T> {

    pub fn dot(self, other: Vector3<T>) -> T {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(self, other: Vector3<T>) -> Vector3<T> {
        return Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        };
    }

    pub fn approx(self, other: Vector3<T>, tolerance: T) -> bool {
        let abs_dx = if self.x > other.x { self.x - other.x } else { other.x - self.x };
        let abs_dy = if self.y > other.y { self.y - other.y } else { other.y - self.y };
        let abs_dz = if self.z > other.z { self.z - other.z } else { other.z - self.z };

        return abs_dx <= tolerance
            && abs_dy <= tolerance
            && abs_dz <= tolerance;
    }
}

impl Vector3<f32> {
    pub fn mag(self) -> f32 {
        return f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }
}

impl Vector3<f64> {
    pub fn mag(self) -> f64 {
        return f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }
}