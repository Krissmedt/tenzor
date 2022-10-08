#[cfg(test)]
mod vector3f_test {
    use crate::vektor::vector3::Vector3;

    #[test]
    fn vectorAddition_shouldReturnCorrectResult_forAllPrimitiveNumericTypes() {
        let operand_i8 = Vector3 {x: 1 as i8, y: 2 as i8, z: 3 as i8};
        let operand_i16 = Vector3 {x: 1 as i16, y: 2 as i16, z: 3 as i16};
        let operand_i32 = Vector3 {x: 1 as i32, y: 2 as i32, z: 3 as i32};
        let operand_i64 = Vector3 {x: 1 as i64, y: 2 as i64, z: 3 as i64};
        let operand_i128 = Vector3 {x: 1 as i128, y: 2 as i128, z: 3 as i128};
        let operand_isize = Vector3 {x: 1 as isize, y: 2 as isize, z: 3 as isize};
        let operand_f32 = Vector3 {x: 1.1 as f32, y: 2.2 as f32, z: 3.3 as f32};
        let operand_f64 = Vector3 {x: 1.1 as f64, y: 2.2 as f64, z: 3.3 as f64};

        let expected_i8 = Vector3 {x: 2 as i8, y: 4 as i8, z: 6 as i8};
        let expected_i16 = Vector3 {x: 2 as i16, y: 4 as i16, z: 6 as i16};
        let expected_i32 = Vector3 {x: 2 as i32, y: 4 as i32, z: 6 as i32};
        let expected_i64 = Vector3 {x: 2 as i64, y: 4 as i64, z: 6 as i64};
        let expected_i128 = Vector3 {x: 2 as i128, y: 4 as i128, z: 6 as i128};
        let expected_isize = Vector3 {x: 2 as isize, y: 4 as isize, z: 6 as isize};
        let expected_f32 = Vector3 {x: 2.2 as f32, y: 4.4 as f32, z: 6.6 as f32};
        let expected_f64 = Vector3 {x: 2.2 as f64, y: 4.4 as f64, z: 6.6 as f64};


        let result_i8 = operand_i8 + operand_i8;
        let result_i16 = operand_i16 + operand_i16;
        let result_i32 = operand_i32 + operand_i32;
        let result_i64 = operand_i64 + operand_i64;
        let result_i128 = operand_i128 + operand_i128;
        let result_isize = operand_isize + operand_isize;
        let result_f32 = operand_f32 + operand_f32;
        let result_f64 = operand_f64 + operand_f64;

        assert_eq!(result_i8, expected_i8);
        assert_eq!(result_i16, expected_i16);
        assert_eq!(result_i32, expected_i32);
        assert_eq!(result_i64, expected_i64);
        assert_eq!(result_i128, expected_i128);
        assert_eq!(result_isize, expected_isize);
        assert_eq!(result_f32, expected_f32);
        assert_eq!(result_f64, expected_f64);
    }

    #[test]
    fn vectorDotProduct_shouldReturnCorrectResult_forAllPrimitiveNumericTypes() {
        let operand_i8 = Vector3 {x: 1 as i8, y: 2 as i8, z: 3 as i8};
        let operand_i16 = Vector3 {x: 1 as i16, y: 2 as i16, z: 3 as i16};
        let operand_i32 = Vector3 {x: 1 as i32, y: 2 as i32, z: 3 as i32};
        let operand_i64 = Vector3 {x: 1 as i64, y: 2 as i64, z: 3 as i64};
        let operand_i128 = Vector3 {x: 1 as i128, y: 2 as i128, z: 3 as i128};
        let operand_isize = Vector3 {x: 1 as isize, y: 2 as isize, z: 3 as isize};
        let operand_f32 = Vector3 {x: 1.1 as f32, y: 2.2 as f32, z: 3.3 as f32};
        let operand_f64 = Vector3 {x: 1.1 as f64, y: 2.2 as f64, z: 3.3 as f64};

        let expected_i8 = 14 as i8;
        let expected_i16 = 14 as i16;
        let expected_i32 = 14 as i32;
        let expected_i64 = 14 as i64;
        let expected_i128 = 14 as i128;
        let expected_isize = 14 as isize;
        let expected_f32 = 16.94 as f32;
        let expected_f64 = 16.94 as f64;


        let result_i8 = operand_i8.dot(operand_i8);
        let result_i16 = operand_i16.dot(operand_i16);
        let result_i32 = operand_i32.dot(operand_i32);
        let result_i64 = operand_i64.dot(operand_i64);
        let result_i128 = operand_i128.dot(operand_i128);
        let result_isize = operand_isize.dot(operand_isize);
        let result_f32 = operand_f32.dot(operand_f32);
        let result_f64 = operand_f64.dot(operand_f64);

        assert_eq!(result_i8, expected_i8);
        assert_eq!(result_i16, expected_i16);
        assert_eq!(result_i32, expected_i32);
        assert_eq!(result_i64, expected_i64);
        assert_eq!(result_i128, expected_i128);
        assert_eq!(result_isize, expected_isize);
        assert_eq!(result_f32, expected_f32);
        assert_eq!(result_f64, expected_f64);
    }
}
