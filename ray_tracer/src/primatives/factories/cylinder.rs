use crate::primatives::Shape;
use crate::primatives::surface::Surface::UnitCylinder;
use math::matrix::matrix_4x4::Matrix4x4;

impl Shape {
    pub fn new_cylinder_transformed(transform: Matrix4x4) -> Self {
        Self::new(transform, UnitCylinder)
    }

    pub fn new_cylinder() -> Self {
        Self::new_cylinder_transformed(Matrix4x4::identity())
    }
}
