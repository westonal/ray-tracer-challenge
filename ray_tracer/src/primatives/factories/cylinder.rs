use crate::primatives::Shape;
use crate::primatives::surface::Surface::UnitCylinder;
use math::matrix::matrix_4x4::Matrix4x4;
use crate::primatives::intersections::CylinderCapStyle;

impl Shape {
    pub fn new_cylinder_transformed(transform: Matrix4x4) -> Self {
        Self::new(transform, UnitCylinder(CylinderCapStyle::Closed))
    }

    pub fn new_open_cylinder_transformed(transform: Matrix4x4) -> Self {
        Self::new(transform, UnitCylinder(CylinderCapStyle::Open))
    }

    pub fn new_cylinder() -> Self {
        Self::new_cylinder_transformed(Matrix4x4::identity())
    }

    pub fn new_open_cylinder() -> Self {
        Self::new_open_cylinder_transformed(Matrix4x4::identity())
    }
}

#[cfg(test)]
mod cylinder_factory_tests {
    use super::*;

    #[test]
    fn cylinders_are_closed_by_default() {
        assert_eq!(UnitCylinder(CylinderCapStyle::Closed), Shape::new_cylinder().surface);
        assert_eq!(UnitCylinder(CylinderCapStyle::Closed), Shape::new_cylinder_transformed(Matrix4x4::identity()).surface);
    }

    #[test]
    fn open_cylinder() {
        assert_eq!(UnitCylinder(CylinderCapStyle::Open), Shape::new_open_cylinder().surface);
        assert_eq!(UnitCylinder(CylinderCapStyle::Open), Shape::new_open_cylinder_transformed(Matrix4x4::identity()).surface);
    }


}