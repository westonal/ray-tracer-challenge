use math::matrix::matrix_4x4::*;
use math::tuple::point::Point;
use math::tuple::vector::Vector;
use std::ops::Deref;

pub struct ViewMatrix(Matrix4x4);

impl ViewMatrix {
    pub fn new_look_at(eye: Point, look_at: Point, up: Vector) -> Self {
        let forward = (look_at - eye).normalize().to_vector();
        let left = forward.cross(up.normalize().to_vector());
        let true_up = left.cross(forward);
        Self(
            Matrix4x4::new([
                [left.x, left.y, left.z, 0.0],
                [true_up.x, true_up.y, true_up.z, 0.0],
                [-forward.x, -forward.y, -forward.z, 0.0],
                [0., 0., 0., 1.],
            ])
            .translation(-eye.x, -eye.y, -eye.z),
        )
    }
}

impl From<ViewMatrix> for Matrix4x4 {
    fn from(value: ViewMatrix) -> Self {
        value.0
    }
}

impl Deref for ViewMatrix {
    type Target = Matrix4x4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod view_matrix_tests {
    use super::*;

    use math::{point, vector};

    #[test]
    fn default_view_matrix() {
        let view_matrix: ViewMatrix =
            ViewMatrix::new_look_at(point!(0, 0, 0), point!(0, 0, -1), vector!(0, 1, 0));
        assert_eq!(Matrix4x4::identity(), view_matrix.0);
    }

    #[test]
    fn a_view_transformation_looking_in_positive_z_direction() {
        let view_matrix =
            ViewMatrix::new_look_at(point!(0, 0, 0), point!(0, 0, 1), vector!(0, 1, 0));
        assert_eq!(Matrix4x4::scale(-1., 1., -1.), view_matrix.0);
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let view_matrix =
            ViewMatrix::new_look_at(point!(0, 0, 8), point!(0, 0, 0), vector!(0, 1, 0));
        assert_eq!(Matrix4x4::translation(0., 0., -8.), view_matrix.0);
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let view_matrix =
            ViewMatrix::new_look_at(point!(1, 3, 2), point!(4, -2, 8), vector!(1, 1, 0));
        assert_eq!(
            Matrix4x4::new([
                [-0.50709254, 0.50709254, 0.6761234, -2.366432],
                [0.76771593, 0.6060915, 0.12121832, -2.828427],
                [-0.35856858, 0.59761435, -0.71713716, -2.3841858e-7],
                [0., 0., 0., 1.]
            ]),
            view_matrix.0
        );
    }
}
