use crate::Angle;
use crate::matrix::matrix_4x4::Matrix4x4;

impl Matrix4x4 {
    pub fn rotation_x(angle: Angle) -> Matrix4x4 {
        let mut m = Self::empty();
        let (sin, cos) = angle.sin_cos();
        m[0][0] = 1.;
        m[1][1] = cos;
        m[2][2] = cos;
        m[2][1] = sin;
        m[1][2] = -sin;
        m[3][3] = 1.;
        m
    }

    pub fn rotation_y(angle: Angle) -> Matrix4x4 {
        let mut m = Self::empty();
        let (sin, cos) = angle.sin_cos();
        m[0][0] = cos;
        m[0][2] = sin;
        m[1][1] = 1.;
        m[2][0] = -sin;
        m[2][2] = cos;
        m[3][3] = 1.;
        m
    }

    pub fn rotation_z(angle: Angle) -> Matrix4x4 {
        let mut m = Self::empty();
        let (sin, cos) = angle.sin_cos();
        m[0][0] = cos;
        m[0][1] = -sin;
        m[1][0] = sin;
        m[1][1] = cos;
        m[2][2] = 1.;
        m[3][3] = 1.;
        m
    }

    pub fn pre_rotation_x(&self, angle: Angle) -> Matrix4x4 {
        self.clone() * Self::rotation_x(angle)
    }

    pub fn pre_rotation_y(&self, angle: Angle) -> Matrix4x4 {
        self.clone() * Self::rotation_y(angle)
    }

    pub fn pre_rotation_z(&self, angle: Angle) -> Matrix4x4 {
        self.clone() * Self::rotation_z(angle)
    }
}

#[cfg(test)]
mod rotation_x_tests {
    use crate::matrix::matrix_4x4::Matrix4x4;
    use crate::tuple::Tuple;

    use crate::{assert_tuple, point, radians};
    use std::f32::consts::PI;

    #[test]
    fn rotate_point_one_eighth() {
        let eighth = Matrix4x4::rotation_x(radians!(PI / 4.0));
        let p = point!(0., 1., 0.);
        assert_eq!(
            eighth * p,
            point!(
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
                std::f32::consts::FRAC_1_SQRT_2
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_eighth_fluent() {
        let eighth = Matrix4x4::identity().pre_rotation_x(radians!(PI / 4.0));
        let p = point!(0., 1., 0.);
        assert_eq!(
            eighth * p,
            point!(
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
                std::f32::consts::FRAC_1_SQRT_2
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_eighth_2() {
        let eighth = Matrix4x4::rotation_x(radians!(PI / 4.0));
        let p = point!(2., 3., 0.);
        assert_eq!(
            eighth * p,
            point!(
                2.,
                3. * std::f32::consts::FRAC_1_SQRT_2,
                3. * std::f32::consts::FRAC_1_SQRT_2
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_quarter() {
        let quarter = Matrix4x4::rotation_x(radians!(PI / 2.0));
        let p = point!(0., 1., 0.);
        assert_tuple!(quarter * p, *point!(0, 0, 1));
    }

    #[test]
    fn rotate_point_one_eighth_inverted() {
        let eighth = Matrix4x4::rotation_x(radians!(PI / 4.0));
        let eighth_reversed = Matrix4x4::rotation_x(radians!(-(PI / 4.0)));
        let eighth_inverted = eighth.invert().unwrap();
        let p = point!(1., 2., 3.);
        assert_tuple!(Tuple::new(1., 3.5355, 0.7071, 1.), eighth_reversed * p);
        assert_tuple!(Tuple::new(1., 3.5355, 0.7071, 1.), eighth_inverted * p);
    }
}

#[cfg(test)]
mod rotation_y_tests {
    use crate::matrix::matrix_4x4::Matrix4x4;
    use crate::tuple::Tuple;

    use crate::{point, radians};
    use std::f32::consts::PI;

    #[test]
    fn rotate_point_one_eighth() {
        let eighth = Matrix4x4::rotation_y(radians!(PI / 4.0));
        let p = point!(0., 0., 1.);
        assert_eq!(
            eighth * p,
            point!(
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
                std::f32::consts::FRAC_1_SQRT_2
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_eighth_fluent() {
        let eighth = Matrix4x4::identity().pre_rotation_y(radians!(PI / 4.0));
        let p = point!(0., 0., 1.);
        assert_eq!(
            eighth * p,
            point!(
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
                std::f32::consts::FRAC_1_SQRT_2
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_eighth_2() {
        let eighth = Matrix4x4::rotation_y(radians!(PI / 4.0));
        let p = point!(2., 3., 0.);
        assert_eq!(
            eighth * p,
            point!(
                2. * std::f32::consts::FRAC_1_SQRT_2,
                3.,
                -2. * std::f32::consts::FRAC_1_SQRT_2
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_quarter() {
        let quarter = Matrix4x4::rotation_y(radians!(PI / 2.0));
        let p = point!(0., 1., 0.);
        assert_eq!(quarter * p, point!(0., 1., 0.).into());
    }

    #[test]
    fn rotate_point_one_eighth_inverted() {
        let eighth = Matrix4x4::rotation_y(radians!(PI / 4.0));
        let eighth_reversed = Matrix4x4::rotation_y(radians!(-(PI / 4.0)));
        let eighth_inverted = eighth.invert().unwrap();
        let p = point!(1., 2., 3.);
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

    use crate::{assert_tuple, point, radians};
    use std::f32::consts::PI;

    #[test]
    fn rotate_point_one_eighth() {
        let eighth = Matrix4x4::rotation_z(radians!(PI / 4.0));
        let p = point!(0., 1., 0.);
        assert_eq!(
            eighth * p,
            point!(
                -std::f32::consts::FRAC_1_SQRT_2,
                std::f32::consts::FRAC_1_SQRT_2,
                0.
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_eighth_fluent() {
        let eighth = Matrix4x4::identity().pre_rotation_z(radians!(PI / 4.0));
        let p = point!(0., 1., 0.);
        assert_eq!(
            eighth * p,
            point!(
                -std::f32::consts::FRAC_1_SQRT_2,
                std::f32::consts::FRAC_1_SQRT_2,
                0.
            )
            .into()
        );
    }

    #[test]
    fn rotate_point_one_eighth_2() {
        let eighth = Matrix4x4::rotation_z(radians!(PI / 4.0));
        let p = point!(2., 3., 2.);
        assert_tuple!(eighth * p, *point!(-0.7071, 3.5355, 2.));
    }

    #[test]
    fn rotate_point_one_quarter() {
        let quarter = Matrix4x4::rotation_z(radians!(PI / 2.0));
        let p = point!(0., 1., 0.);
        assert_tuple!(quarter * p, *point!(-1, 0, 0));
    }

    #[test]
    fn rotate_point_one_eighth_inverted() {
        let eighth = Matrix4x4::rotation_z(radians!(PI / 4.0));
        let eighth_reversed = Matrix4x4::rotation_z(radians!(-(PI / 4.0)));
        let eighth_inverted = eighth.invert().unwrap();
        let p = point!(1., 2., 3.);
        assert_tuple!(Tuple::new(2.1213, 0.7071, 3., 1.), eighth_reversed * p);
        assert_tuple!(Tuple::new(2.1213, 0.7071, 3., 1.), eighth_inverted * p);
    }
}
