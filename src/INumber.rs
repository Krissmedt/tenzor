use std::ops;

pub trait INumber:
Copy
+ ops::Add<Self, Output = Self>
+ ops::Sub<Self, Output = Self>
+ ops::Div<Self, Output = Self>
+ ops::Mul<Self, Output = Self>
+ ops::Rem<Self, Output = Self>
{}