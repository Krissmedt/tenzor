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

        let expected_i8 = "[1, 2, 3]";
        let expected_i16 = Vector3 {x: 2 as i16, y: 4 as i16, z: 6 as i16};
        let expected_i32 = Vector3 {x: 2 as i32, y: 4 as i32, z: 6 as i32};
        let expected_i64 = Vector3 {x: 2 as i64, y: 4 as i64, z: 6 as i64};
        let expected_i128 = Vector3 {x: 2 as i128, y: 4 as i128, z: 6 as i128};
        let expected_isize = Vector3 {x: 2 as isize, y: 4 as isize, z: 6 as isize};
        let expected_f32 = Vector3 {x: 2.2 as f32, y: 4.4 as f32, z: 6.6 as f32};
        let expected_f64 = Vector3 {x: 2.2 as f64, y: 4.4 as f64, z: 6.6 as f64};

        let result_i8 = "{}";
    }
}