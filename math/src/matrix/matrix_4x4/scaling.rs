use crate::matrix::matrix_4x4::Matrix4x4;

impl Matrix4x4 {
    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4x4 {
        let mut m = Self::empty();
        m[0][0] = x;
        m[1][1] = y;
        m[2][2] = z;
        m[3][3] = 1.;
        m
    }

    pub fn pre_scale(&self, x: f32, y: f32, z: f32) -> Matrix4x4 {
        self.clone() * Matrix4x4::scale(x, y, z)
    }
}

#[cfg(test)]
mod scale_tests {

    use crate::matrix::matrix_4x4::Matrix4x4;
    use crate::tuple::point::Point;
    use crate::tuple::vector::Vector;

    #[test]
    fn scale_point() {
        let m = Matrix4x4::scale(2., 3., 4.);
        let p = Point::point(-4., 6., 8.);
        assert_eq!(m * p, Point::point(-8., 18., 32.).into());
    }

    #[test]
    fn scale_point_fluent() {
        let m = Matrix4x4::identity().pre_scale(2., 3., 4.);
        let p = Point::point(-4., 6., 8.);
        assert_eq!(m * p, Point::point(-8., 18., 32.).into());
    }

    #[test]
    fn scale_vector() {
        let m = Matrix4x4::scale(2., 3., 4.);
        let p = Vector::vector(-4., 6., 8.);
        assert_eq!(m * p, Vector::vector(-8., 18., 32.).into())
    }

    #[test]
    fn multiply_by_inverse_of_scale() {
        let m = Matrix4x4::scale(2., 3., 4.).invert().unwrap();
        let p = Vector::vector(-4., 6., 8.);
        assert_eq!(m * p, Vector::vector(-2., 2., 2.).into())
    }

    #[test]
    fn reflect_in_x() {
        let m = Matrix4x4::scale(-1., 1., 1.);
        let p = Vector::vector(2., 3., 4.);
        assert_eq!(m * p, Vector::vector(-2., 3., 4.).into())
    }
}
