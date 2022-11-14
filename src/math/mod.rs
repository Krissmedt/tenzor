pub mod IRealNumber;

pub fn approx<T : IRealNumber::IRealNumber>(a : T, b : T, tolerance : T) -> bool {
    let abs = if a > b {a - b} else {b - a};
    return abs <= tolerance
}