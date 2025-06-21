use crate::matrix::matrix_4x4::Matrix4x4;

impl Matrix4x4 {
    pub fn rotation_x(x_radians: f32) -> Matrix4x4 {
        let mut m = Self::empty();
        let (sin, cos) = x_radians.sin_cos();
        m[0][0] = 1.;
        m[1][1] = cos;
        m[2][2] = cos;
        m[2][1] = sin;
        m[1][2] = -sin;
        m[3][3] = 1.;
        m
    }

    pub fn rotation_y(y_radians: f32) -> Matrix4x4 {
        let mut m = Self::empty();
        let (sin, cos) = y_radians.sin_cos();
        m[0][0] = cos;
        m[0][2] = sin;
        m[1][1] = 1.;
        m[2][0] = -sin;
        m[2][2] = cos;
        m[3][3] = 1.;
        m
    }

    pub fn rotation_z(z_radians: f32) -> Matrix4x4 {
        let mut m = Self::empty();
        let (sin, cos) = z_radians.sin_cos();
        m[0][0] = cos;
        m[0][1] = -sin;
        m[1][0] = sin;
        m[1][1] = cos;
        m[2][2] = 1.;
        m[3][3] = 1.;
        m
    }
}

#[cfg(test)]
mod rotation_x_tests {
    use crate::matrix::matrix_4x4::Matrix4x4;
    use crate::tuple::Tuple;
    use crate::tuple::point::Point;

    use std::f32::consts::PI;

    #[test]
    fn rotate_point_one_eighth() {
        let eighth = Matrix4x4::rotation_x(PI / 4.0);
        let p = Point::point(0., 1., 0.);
        assert_eq!(
            eighth * p,
            Point::point(
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
                std::f32::consts::FRAC_1_SQRT_2
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_eighth_2() {
        let eighth = Matrix4x4::rotation_x(PI / 4.0);
        let p = Point::point(2., 3., 0.);
        assert_eq!(
            eighth * p,
            Point::point(
                2.,
                3. * std::f32::consts::FRAC_1_SQRT_2,
                3. * std::f32::consts::FRAC_1_SQRT_2
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_quarter() {
        let quarter = Matrix4x4::rotation_x(PI / 2.0);
        let p = Point::point(0., 1., 0.);
        assert_eq!(quarter * p, Point::point(0., -4.371139e-8, 1.).into());
    }

    #[test]
    fn rotate_point_one_eighth_inverted() {
        let eighth = Matrix4x4::rotation_x(PI / 4.0);
        let eighth_reversed = Matrix4x4::rotation_x(-(PI / 4.0));
        let eighth_inverted = eighth.invert().unwrap();
        let p = Point::point(1., 2., 3.);
        assert_eq!(Tuple::new(1., 3.535534, 0.7071067, 1.), eighth_reversed * p);
        assert_eq!(
            Tuple::new(1., 3.5355341, 0.7071068, 1.),
            eighth_inverted * p
        );
    }
}

#[cfg(test)]
mod rotation_y_tests {
    use crate::matrix::matrix_4x4::Matrix4x4;
    use crate::tuple::Tuple;
    use crate::tuple::point::Point;

    use std::f32::consts::PI;

    #[test]
    fn rotate_point_one_eighth() {
        let eighth = Matrix4x4::rotation_y(PI / 4.0);
        let p = Point::point(0., 0., 1.);
        assert_eq!(
            eighth * p,
            Point::point(
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
                std::f32::consts::FRAC_1_SQRT_2
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_eighth_2() {
        let eighth = Matrix4x4::rotation_y(PI / 4.0);
        let p = Point::point(2., 3., 0.);
        assert_eq!(
            eighth * p,
            Point::point(
                2. * std::f32::consts::FRAC_1_SQRT_2,
                3.,
                -2. * std::f32::consts::FRAC_1_SQRT_2
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_quarter() {
        let quarter = Matrix4x4::rotation_y(PI / 2.0);
        let p = Point::point(0., 1., 0.);
        assert_eq!(quarter * p, Point::point(0., 1., 0.).into());
    }

    #[test]
    fn rotate_point_one_eighth_inverted() {
        let eighth = Matrix4x4::rotation_y(PI / 4.0);
        let eighth_reversed = Matrix4x4::rotation_y(-(PI / 4.0));
        let eighth_inverted = eighth.invert().unwrap();
        let p = Point::point(1., 2., 3.);
        assert_eq!(
            Tuple::new(-1.4142134, 2., 2.828427, 1.),
            eighth_reversed * p
        );
        assert_eq!(
            Tuple::new(-1.4142137, 2., 2.8284273, 1.),
            eighth_inverted * p
        );
    }
}

#[cfg(test)]
mod rotation_z_tests {
    use crate::matrix::matrix_4x4::Matrix4x4;
    use crate::tuple::Tuple;
    use crate::tuple::point::Point;

    use std::f32::consts::PI;

    #[test]
    fn rotate_point_one_eighth() {
        let eighth = Matrix4x4::rotation_z(PI / 4.0);
        let p = Point::point(0., 1., 0.);
        assert_eq!(
            eighth * p,
            Point::point(
                -std::f32::consts::FRAC_1_SQRT_2,
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_eighth_2() {
        let eighth = Matrix4x4::rotation_z(PI / 4.0);
        let p = Point::point(2., 3., 2.);
        assert_eq!(eighth * p, Point::point(-0.7071067, 3.535534, 2.,).into());
    }

    #[test]
    fn rotate_point_one_quarter() {
        let quarter = Matrix4x4::rotation_z(PI / 2.0);
        let p = Point::point(0., 1., 0.);
        assert_eq!(quarter * p, Point::point(-1., -4.371139e-8, 0.).into());
    }

    #[test]
    fn rotate_point_one_eighth_inverted() {
        let eighth = Matrix4x4::rotation_z(PI / 4.0);
        let eighth_reversed = Matrix4x4::rotation_z(-(PI / 4.0));
        let eighth_inverted = eighth.invert().unwrap();
        let p = Point::point(1., 2., 3.);
        assert_eq!(
            Tuple::new(2.1213202, 0.70710677, 3., 1.),
            eighth_reversed * p
        );
        assert_eq!(
            Tuple::new(2.1213205, 0.7071068, 3., 1.),
            eighth_inverted * p
        );
    }
}
