#[cfg(test)]
mod chain_tests {

    use crate::matrix::matrix_4x4::Matrix4x4;
    use crate::tuple::point::Point;
    use std::f32::consts::PI;

    #[test]
    fn chain() {
        assert_eq!(
            Matrix4x4::translation(10., 5., 7.)
                * Matrix4x4::scale(5., 5., 5.,)
                * Matrix4x4::rotation_x(PI / 2.)
                * Point::point(1., 0., 1.),
            Point::point(15., 0., 7.).into()
        )
    }

    #[test]
    fn chain_reversed() {
        assert_eq!(
            Matrix4x4::rotation_x(PI / 2.)
                * Matrix4x4::scale(5., 5., 5.,)
                * Matrix4x4::translation(10., 5., 7.)
                * Point::point(1., 0., 1.),
            Point::point(55., -40., 24.999998).into()
        )
    }

    #[test]
    fn chain_fluent() {
        assert_eq!(
            Matrix4x4::translation(10., 5., 7.)
                .pre_scale(5., 5., 5.,)
                .pre_rotation_x(PI / 2.)
                * Point::point(1., 0., 1.),
            Point::point(15., 0., 7.).into()
        )
    }

    #[test]
    fn chain_fluent_reversed() {
        assert_eq!(
            Matrix4x4::rotation_x(PI / 2.)
                .pre_scale(5., 5., 5.,)
                .pre_translation(10., 5., 7.)
                * Point::point(1., 0., 1.),
            Point::point(55., -40., 24.999998).into()
        )
    }
}
