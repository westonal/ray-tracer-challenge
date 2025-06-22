use crate::rays::Ray;
use math::matrix::matrix_4x4::Matrix4x4;
use std::ops::Mul;

impl Mul<Ray> for Matrix4x4 {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        let point = (self * rhs.origin)
            .try_into()
            .expect("Matrix supplied mapped point to non-point");
        let direction = (self * rhs.direction)
            .try_into()
            .expect("Matrix supplied mapped point to non-vector");
        Ray::new(point, direction)
    }
}

#[cfg(test)]
mod ray_transform_tests {
    use super::*;
    use crate::ray;
    use math::matrix::matrix_4x4::Matrix4x4;

    #[test]
    fn translate_a_point_in_a_ray() {
        let ray = ray!((1., 2., 3.), (0., 1., 0.));
        assert_eq!(
            Matrix4x4::translation(3., 4., 5.) * ray,
            ray!((4., 6., 8.), (0., 1., 0.))
        )
    }

    #[test]
    fn scale_a_ray() {
        let ray = ray!((1., 2., 3.), (0., 1., 0.));
        assert_eq!(
            Matrix4x4::scale(2., 3., 4.) * ray,
            ray!((2., 6., 12.), (0., 3., 0.))
        )
    }
}
