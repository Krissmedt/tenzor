#[cfg(test)]
mod vector3f_test {
    use crate::vektor::vector3::Vector3;

    #[test]
    fn addToVector_shouldReturnVector_whenBothOperandsAreIntegerVectors() {
        let a = Vector3 {x :1, y: 2, z: 3 };
        let b = Vector3 {x :4, y: 5, z: 6 };
        let expected = Vector3 {x :5, y: 7, z: 9 };

        let result = a + b;

        assert_eq!(result, expected);
    }
}
