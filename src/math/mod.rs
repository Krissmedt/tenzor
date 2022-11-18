mod math_test;

use std::fmt::Display;
use std::ops;

pub trait IRealNumber:
Copy
+ Display
+ ops::Add<Self, Output = Self>
+ ops::AddAssign<Self>
+ ops::Sub<Self, Output = Self>
+ ops::SubAssign<Self>
+ ops::Div<Self, Output = Self>
+ ops::DivAssign<Self>
+ ops::Mul<Self, Output = Self>
+ ops::MulAssign<Self>
+ ops::Rem<Self, Output = Self>
+ ops::RemAssign<Self>
+ ops::Neg<Output = Self>
+ PartialOrd
+ From<i8>
{}

impl IRealNumber for i8 {}
impl IRealNumber for i16 {}
impl IRealNumber for i32 {}
impl IRealNumber for i64 {}
impl IRealNumber for i128 {}
impl IRealNumber for isize {}

impl IRealNumber for f32 {}
impl IRealNumber for f64 {}


pub fn abs<T : IRealNumber>(a : T) -> T {
    return if a >= T::from(0) {a} else {-a}
}

pub fn approx<T : IRealNumber>(a : T, b : T, tolerance : T) -> bool {
    let abs = if a > b {a - b} else {b - a};
    return abs <= tolerance
}