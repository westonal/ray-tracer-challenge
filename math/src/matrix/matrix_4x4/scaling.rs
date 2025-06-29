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

    pub fn scale_all(scale: f32) -> Matrix4x4 {
        Self::scale(scale, scale, scale)
    }

    pub fn pre_scale(&self, x: f32, y: f32, z: f32) -> Matrix4x4 {
        self.clone() * Matrix4x4::scale(x, y, z)
    }

    pub fn pre_scale_all(&self, scale: f32) -> Matrix4x4 {
        self.pre_scale(scale, scale, scale)
    }
}

#[cfg(test)]
mod scale_tests {

    use crate::matrix::matrix_4x4::Matrix4x4;

    use crate::{point, vector};

    #[test]
    fn scale_point() {
        let m = Matrix4x4::scale(2., 3., 4.);
        let p = point!(-4., 6., 8.);
        assert_eq!(m * p, point!(-8., 18., 32.).into());
    }

    #[test]
    fn scale_all_point() {
        let m = Matrix4x4::scale_all(2.);
        let p = point!(-4., 6., 8.);
        assert_eq!(m * p, point!(-8., 12., 16.).into());
    }

    #[test]
    fn scale_point_fluent() {
        let m = Matrix4x4::identity().pre_scale(2., 3., 4.);
        let p = point!(-4., 6., 8.);
        assert_eq!(m * p, point!(-8., 18., 32.).into());
    }

    #[test]
    fn scale_all_point_fluent() {
        let m = Matrix4x4::identity().pre_scale_all(2.);
        let p = point!(-4., 6., 8.);
        assert_eq!(m * p, point!(-8., 12., 16.).into());
    }

    #[test]
    fn scale_vector() {
        let m = Matrix4x4::scale(2., 3., 4.);
        let p = vector!(-4., 6., 8.);
        assert_eq!(m * p, vector!(-8., 18., 32.).into())
    }

    #[test]
    fn multiply_by_inverse_of_scale() {
        let m = Matrix4x4::scale(2., 3., 4.).invert().unwrap();
        let p = vector!(-4., 6., 8.);
        assert_eq!(m * p, vector!(-2., 2., 2.).into())
    }

    #[test]
    fn reflect_in_x() {
        let m = Matrix4x4::scale(-1., 1., 1.);
        let p = vector!(2., 3., 4.);
        assert_eq!(m * p, vector!(-2., 3., 4.).into())
    }
}
