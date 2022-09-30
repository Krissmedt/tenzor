use crate::INumber::INumber;

#[derive(Debug, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

use std::ops;

// impl<T : INumber> PartialEq<Vector3<T>> for Vector3<T> {
//     fn eq(&self, other: &Vector3<T>) -> bool {
//         let tolerance = 0.01;
//         let rel_delta_x = (self.x - other.x)/self.x;
//         let rel_delta_y = (self.y - other.y)/self.y;
//         let rel_delta_z = (self.z - other.z)/self.z;
//         return rel_delta_x <= tolerance
//             && rel_delta_y <= tolerance
//             && rel_delta_z <= tolerance
//     }
// }

impl<T : INumber> ops::Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, _rhs: Vector3<T>) -> Vector3<T> {
        return Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl<T : INumber> Vector3<T> {
    fn dot(&self, other : &Vector3<T>) -> T {
        return self.x*other.x + self.y*other.y + self.z*other.z
    }

}