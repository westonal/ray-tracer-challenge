#[cfg(test)]
mod chain_tests {

    use crate::matrix::matrix_4x4::*;
    use crate::{point, radians};

    use std::f32::consts::PI;

    #[test]
    fn chain() {
        assert_eq!(
            Matrix4x4::translation(10., 5., 7.)
                * Matrix4x4::scale_all(5.)
                * Matrix4x4::rotation_x(radians!(PI / 2.))
                * point!(1., 0., 1.),
            point!(15., 0., 7.).into()
        )
    }

    #[test]
    fn chain_reversed() {
        assert_eq!(
            Matrix4x4::rotation_x(radians!(PI / 2.))
                * Matrix4x4::scale_all(5.)
                * Matrix4x4::translation(10., 5., 7.)
                * point!(1., 0., 1.),
            point!(55., -40., 24.999998).into()
        )
    }

    #[test]
    fn chain_fluent() {
        assert_eq!(
            Matrix4x4::translation(10., 5., 7.)
                .scale_all(5.)
                .rotation_x(radians!(PI / 2.))
                * point!(1., 0., 1.),
            point!(15., 0., 7.).into()
        )
    }

    #[test]
    fn chain_fluent_reversed() {
        assert_eq!(
            Matrix4x4::rotation_x(radians!(PI / 2.))
                .scale_all(5.)
                .translation(10., 5., 7.)
                * point!(1., 0., 1.),
            point!(55., -40., 24.999998).into()
        )
    }
}
