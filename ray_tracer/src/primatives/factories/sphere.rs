use crate::primatives::Shape;
use crate::primatives::surface::Surface::UnitSphere;
use math::matrix::matrix_4x4::Matrix4x4;

impl Shape {
    pub fn new_sphere_transformed(transform: Matrix4x4) -> Self {
        Self::new(transform, UnitSphere)
    }

    pub fn new_sphere() -> Self {
        Self::new_sphere_transformed(Matrix4x4::identity())
    }
}
