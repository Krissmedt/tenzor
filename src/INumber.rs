use num_traits::{Num, NumOps, RefNum};

pub trait INumber : Num + NumOps + Copy {}