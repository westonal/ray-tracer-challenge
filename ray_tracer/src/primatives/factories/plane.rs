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
