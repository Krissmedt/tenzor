#[cfg(test)]
mod vector3f_test {
    use crate::vektor::vector3f::Vector3f;

    #[test]
    fn addToVector_shouldReturnVector() {
        let a = Vector3f {x :1.1, y: 2.2, z: 3.3 };
        let b = Vector3f {x :4.4, y: 5.5, z: 6.6 };
        let expected = Vector3f {x :5.5, y: 7.7, z: 9.9 };

        let result = a + b;

        assert_eq!(result, expected);
    }
}
