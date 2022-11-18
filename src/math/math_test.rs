#[cfg(test)]
mod math_test {
    use crate::math;

    #[test]
    fn abs_shouldReturnAbsoluteValue_forAllPositivePrimitiveNumericTypes() {
        let positive_i8 = 2 as i8;
        let positive_i16 = 2 as i16;
        let positive_i32 = 2 as i32;
        let positive_i64 = 2 as i64;
        let positive_i128 = 2 as i128;
        let positive_isize = 2 as isize;
        let positive_f32 = 2 as f32;
        let positive_f64 = 2 as f64;

        assert_eq!(math::abs(positive_i8), positive_i8);
        assert_eq!(math::abs(positive_i16), positive_i16);
        assert_eq!(math::abs(positive_i32), positive_i32);
        assert_eq!(math::abs(positive_i64), positive_i64);
        assert_eq!(math::abs(positive_i128), positive_i128);
        assert_eq!(math::abs(positive_isize), positive_isize);
        assert_eq!(math::abs(positive_f32), positive_f32);
        assert_eq!(math::abs(positive_f64), positive_f64);
    }

    #[test]
    fn abs_shouldReturnAbsoluteValue_forAllNegativePrimitiveNumericTypes() {
        let negative_i8 = -2 as i8;
        let negative_i16 = -2 as i16;
        let negative_i32 = -2 as i32;
        let negative_i64 = -2 as i64;
        let negative_i128 = -2 as i128;
        let negative_isize = -2 as isize;
        let negative_f32 = -2.3 as f32;
        let negative_f64 = -2.3 as f64;

        assert_eq!(math::abs(negative_i8), 2);
        assert_eq!(math::abs(negative_i16), 2);
        assert_eq!(math::abs(negative_i32), 2);
        assert_eq!(math::abs(negative_i64), 2);
        assert_eq!(math::abs(negative_i128), 2);
        assert_eq!(math::abs(negative_isize), 2);
        assert_eq!(math::abs(negative_f32), 2.3);
        assert_eq!(math::abs(negative_f64), 2.3);
    }

    #[test]
    fn approx_shouldReturnTrue_whenOperandWithinTolerance_forAllPrimitiveNumericTypes() {
        assert!(math::approx(10 as i8, 11 as i8, 2 as i8));
        assert!(math::approx(10 as i16, 11 as i16, 2 as i16));
        assert!(math::approx(10 as i32, 11 as i32, 2 as i32));
        assert!(math::approx(10 as i64, 11 as i64, 2 as i64));
        assert!(math::approx(10 as i128, 11 as i128, 2 as i128));
        assert!(math::approx(10 as isize, 11 as isize, 2 as isize));
        assert!(math::approx(10.1 as f32, 10.2 as f32, 0.2 as f32));
        assert!(math::approx(10.1 as f64, 10.2 as f64, 0.2 as f64));
    }

    #[test]
    fn approx_shouldReturnFalse_whenOperandNotWithinTolerance_forAllPrimitiveNumericTypes() {
        assert!(!math::approx(10 as i8, 12 as i8, 1 as i8));
        assert!(!math::approx(10 as i16, 12 as i16, 1 as i16));
        assert!(!math::approx(10 as i32, 12 as i32, 1 as i32));
        assert!(!math::approx(10 as i64, 12 as i64, 1 as i64));
        assert!(!math::approx(10 as i128, 12 as i128, 1 as i128));
        assert!(!math::approx(10 as isize, 12 as isize, 1 as isize));
        assert!(!math::approx(10.1 as f32, 10.2 as f32, 0.02 as f32));
        assert!(!math::approx(10.1 as f64, 10.2 as f64, 0.02 as f64));
    }
}