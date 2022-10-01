#[derive(Debug)]
pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

use std::ops;

impl PartialEq<Vector3f> for Vector3f {
    fn eq(&self, other: &Vector3f) -> bool {
        let tolerance = 0.01;
        let rel_delta_x = (self.x - other.x)/self.x;
        let rel_delta_y = (self.y - other.y)/self.y;
        let rel_delta_z = (self.z - other.z)/self.z;
        return rel_delta_x <= tolerance
            && rel_delta_y <= tolerance
            && rel_delta_z <= tolerance
    }
}

impl ops::Add<Vector3f> for Vector3f {
    type Output = Vector3f;

    fn add(self, _rhs: Vector3f) -> Vector3f {
        return Vector3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl Vector3f {
    fn dot(&self, other : &Vector3f) -> f64 {
        return self.x*other.x + self.y*other.y + self.z*other.z
    }

}