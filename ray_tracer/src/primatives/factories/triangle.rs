use crate::primatives::Shape;
use crate::primatives::surface::Surface::SingleTriangle;
use crate::primatives::triangle::Triangle;
use math::matrix::matrix_4x4::*;

impl Shape {
    pub fn new_triangle_transformed(transform: Matrix4x4, triangle: Triangle) -> Self {
        Self::new(transform, SingleTriangle(triangle))
    }

    pub fn new_triangle(triangle: Triangle) -> Self {
        Self::new_triangle_transformed(Matrix4x4::identity(), triangle)
    }
}
