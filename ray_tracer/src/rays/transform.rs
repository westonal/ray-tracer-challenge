use crate::rays::Ray;
use math::matrix::matrix_4x4::Matrix4x4;

impl Ray {
    pub fn transform(&self, m: Matrix4x4) -> Self {
        let point = (m * self.origin)
            .try_into()
            .expect("Matrix supplied mapped point to non-point");
        let direction = (m * self.direction)
            .try_into()
            .expect("Matrix supplied mapped point to non-vector");
        Self::new(point, direction)
    }
}

#[cfg(test)]
mod ray_transform_tests {
    use crate::ray;
    use math::matrix::matrix_4x4::Matrix4x4;

    #[test]
    fn translate_a_point_in_a_ray() {
        let ray = ray!((1., 2., 3.), (0., 1., 0.));
        assert_eq!(
            ray.transform(Matrix4x4::translation(3., 4., 5.)),
            ray!((4., 6., 8.), (0., 1., 0.))
        )
    }

    #[test]
    fn scale_a_ray() {
        let ray = ray!((1., 2., 3.), (0., 1., 0.));
        assert_eq!(
            ray.transform(Matrix4x4::scale(2., 3., 4.)),
            ray!((2., 6., 12.), (0., 3., 0.))
        )
    }
}
