use crate::IRealNumber::IRealNumber;
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T : IRealNumber> ops::Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, _rhs: Vector3<T>) -> Vector3<T> {
        return Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl<T : IRealNumber> Vector3<T> {
    pub fn dot(self, other : Vector3<T>) -> T {
        return self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn approx(self, other : Vector3<T>, tolerance : T) -> bool {
        false
    }
}