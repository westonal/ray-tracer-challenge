use crate::primatives::Shape;
use crate::primatives::surface::Surface::UnitCube;
use math::matrix::matrix_4x4::Matrix4x4;

impl Shape {
    pub fn new_cube_transformed(transform: Matrix4x4) -> Self {
        Self::new(transform, UnitCube)
    }

    pub fn new_cube() -> Self {
        Self::new_cube_transformed(Matrix4x4::identity())
    }
}
