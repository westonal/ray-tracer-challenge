use crate::matrix::matrix_4x4::*;

pub trait Matrix4x4Shear {
    fn shear(self, x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix4x4;
}

impl Matrix4x4Shear for Matrix4x4 {
    fn shear(self, x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix4x4 {
        self * Matrix4x4::shear(x_y, x_z, y_x, y_z, z_x, z_y)
    }
}

impl Matrix4x4 {
    pub fn shear(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix4x4 {
        let mut m = Self::identity();
        m[0][1] = x_y;
        m[0][2] = x_z;
        m[1][0] = y_x;
        m[1][2] = y_z;
        m[2][0] = z_x;
        m[2][1] = z_y;
        m
    }
}

#[cfg(test)]
mod matrix_4x4_shearing_tests {
    use crate::matrix::matrix_4x4::*;
    use crate::point;

    #[test]
    fn shear_x_in_proportion_to_y() {
        let m = Matrix4x4::shear(1., 0., 0., 0., 0., 0.);
        let p = point!(2., 3., 4.);
        assert_eq!(m * p, point!(5., 3., 4.).into());
    }

    #[test]
    fn shear_x_in_proportion_to_z() {
        let m = Matrix4x4::shear(0., 1., 0., 0., 0., 0.);
        let p = point!(2., 3., 4.);
        assert_eq!(m * p, point!(6., 3., 4.).into());
    }

    #[test]
    fn shear_y_in_proportion_to_x() {
        let m = Matrix4x4::shear(0., 0., 1., 0., 0., 0.);
        let p = point!(2., 3., 4.);
        assert_eq!(m * p, point!(2., 5., 4.).into());
    }

    #[test]
    fn shear_y_in_proportion_to_z() {
        let m = Matrix4x4::shear(0., 0., 0., 1., 0., 0.);
        let p = point!(2., 3., 4.);
        assert_eq!(m * p, point!(2., 7., 4.).into());
    }

    #[test]
    fn shear_x_in_proportion_to_x() {
        let m = Matrix4x4::shear(0., 0., 0., 0., 1., 0.);
        let p = point!(2., 3., 4.);
        assert_eq!(m * p, point!(2., 3., 6.).into());
    }

    #[test]
    fn shear_z_in_proportion_to_y() {
        let m = Matrix4x4::shear(0., 0., 0., 0., 0., 1.);
        let p = point!(2., 3., 4.);
        assert_eq!(m * p, point!(2., 3., 7.).into());
    }

    #[test]
    fn shear_fluent() {
        let m = Matrix4x4::translation(3., 2., 5.).shear(1., 2., 3., 4., 5., 6.);
        let p = point!(2., 3., 4.);
        assert_eq!(m * p, point!(16., 27., 37.).into());
    }
}
