use std::ops;

pub trait IRealNumber:
Copy
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
+ ops::Neg
+ PartialOrd
{}

impl IRealNumber for i8 {}
impl IRealNumber for i16 {}
impl IRealNumber for i32 {}
impl IRealNumber for i64 {}
impl IRealNumber for i128 {}
impl IRealNumber for isize {}

impl IRealNumber for f32 {}
impl IRealNumber for f64 {}