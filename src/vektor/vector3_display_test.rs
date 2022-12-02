#[cfg(test)]
#[allow(non_snake_case)]
mod vector3_display_test {
    use crate::vektor::vector3::Vector3;

    #[test]
    fn stringFormatting_shouldReturnCorrectVector3View_forAllPrimitiveNumericTypes() {
        let operand_i8 = Vector3 {x: 1 as i8, y: 2 as i8, z: 3 as i8};
        let operand_i16 = Vector3 {x: 1 as i16, y: 2 as i16, z: 3 as i16};
        let operand_i32 = Vector3 {x: 1 as i32, y: 2 as i32, z: 3 as i32};
        let operand_i64 = Vector3 {x: 1 as i64, y: 2 as i64, z: 3 as i64};
        let operand_i128 = Vector3 {x: 1 as i128, y: 2 as i128, z: 3 as i128};
        let operand_isize = Vector3 {x: 1 as isize, y: 2 as isize, z: 3 as isize};
        let operand_f32 = Vector3 {x: 1.1 as f32, y: 2.2 as f32, z: 3.3 as f32};
        let operand_f64 = Vector3 {x: 1.1 as f64, y: 2.2 as f64, z: 3.3 as f64};

        let expected_integer = "[1, 2, 3]";
        let expected_float = "[1.1, 2.2, 3.3]";

        let result_i8 = format!("{}", operand_i8);
        let result_i16 = format!("{}", operand_i16);
        let result_i32 = format!("{}", operand_i32);
        let result_i64 = format!("{}", operand_i64);
        let result_i128 = format!("{}", operand_i128);
        let result_isize = format!("{}", operand_isize);
        let result_f32 = format!("{}", operand_f32);
        let result_f64 = format!("{}", operand_f64);

        assert_eq!(result_i8, expected_integer);
        assert_eq!(result_i16, expected_integer);
        assert_eq!(result_i32, expected_integer);
        assert_eq!(result_i64, expected_integer);
        assert_eq!(result_i128, expected_integer);
        assert_eq!(result_isize, expected_integer);
        assert_eq!(result_f32, expected_float);
        assert_eq!(result_f64, expected_float);
    }
}