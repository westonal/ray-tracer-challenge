use crate::primatives::Shape;
use crate::primatives::surface::Surface::PlaneXZ;
use math::matrix::matrix_4x4::Matrix4x4;

impl Shape {
    pub fn new_plane_transformed(transform: Matrix4x4) -> Self {
        Self::new(transform, PlaneXZ)
    }

    pub fn new_plane() -> Self {
        Self::new_plane_transformed(Matrix4x4::identity())
    }
}

#[cfg(test)]
mod plane_normal_tests {
    use crate::primatives::Shape;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::tuple::point::Point;
    use math::{point, radians, vector};
    use std::f32::consts::PI;

    #[test]
    fn plane_normal() {
        let plane = Shape::new_plane();
        assert_eq!(vector!(0, 1, 0), *plane.normal_at(Point::origin()));
        assert_eq!(vector!(0, 1, 0), *plane.normal_at(point!(1, 2, 3)));
    }

    #[test]
    fn plane_normal_transformed() {
        let plane = Shape::new_plane_transformed(Matrix4x4::rotation_z(radians!(PI / 2.)));
        assert_eq!(
            vector!(-1, -4.371139e-8, 0),
            *plane.normal_at(Point::origin())
        );
        assert_eq!(
            vector!(-1, -4.371139e-8, 0),
            *plane.normal_at(point!(1, 2, 3))
        );
    }
}
