use crate::primatives::intersections::Sphere;
use crate::primatives::intersections::{Cube, Cylinder};
use crate::primatives::intersections::{CylinderCapStyle, Plane};
use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;
use crate::primatives::triangle::Triangle;

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum Surface {
    /// A sphere with center at origin and radius 1
    UnitSphere,

    /// AABB Axis-Aligned Bounding Box
    UnitCube,

    /// The Plane Y=0
    PlaneXZ,

    /// A cylinder with center at origin and radius 1, infinite length
    UnitCylinder(CylinderCapStyle),

    SingleTriangle(Triangle)
}

impl Surface {
    pub(crate) fn fast_hit(&self, ray: &Ray) -> bool {
        match self {
            Surface::UnitSphere => !Sphere::intersect(ray).is_empty(),
            Surface::PlaneXZ => !Plane::intersect(ray).is_empty(),
            Surface::UnitCube => Cube::fast_hit(ray),
            Surface::UnitCylinder(style) => !Cylinder::intersect(ray, style).is_empty(),
            &Surface::SingleTriangle(triangle) => !triangle.intersect(ray).is_empty(),
        }
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<f32> {
        match self {
            Surface::UnitSphere => Sphere::intersect(ray),
            Surface::PlaneXZ => Plane::intersect(ray),
            Surface::UnitCube => Cube::intersect(ray),
            Surface::UnitCylinder(style) => Cylinder::intersect(ray, style),
            &Surface::SingleTriangle(triangle) => triangle.intersect(ray),
        }
    }

    pub(crate) fn normal_at(&self, object_point: Point) -> Vector {
        match self {
            Surface::UnitSphere => Sphere::normal_at(object_point),
            Surface::PlaneXZ => Plane::normal_at(object_point),
            Surface::UnitCube => Cube::normal_at(object_point),
            Surface::UnitCylinder(style) => Cylinder::normal_at(object_point, style),
            &Surface::SingleTriangle(triangle) => triangle.normal_at(object_point),
        }
    }
}
