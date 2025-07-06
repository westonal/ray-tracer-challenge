use crate::primatives::intersections::Plane;
use crate::primatives::intersections::Sphere;
use crate::primatives::intersections::{Cube, Cylinder};
use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;

#[derive(PartialEq, Debug)]
pub(crate) enum Surface {
    /// A sphere with center at origin and radius 1
    UnitSphere,

    /// AABB Axis-Aligned Bounding Box
    UnitCube,

    /// The Plane Y=0
    PlaneXZ,

    /// A cylinder with center at origin and radius 1, infinite length
    UnitCylinder,
}

impl Surface {
    pub(crate) fn intersect(&self, ray: Ray) -> Vec<f32> {
        match self {
            Surface::UnitSphere => Sphere::intersect(ray),
            Surface::PlaneXZ => Plane::intersect(ray),
            Surface::UnitCube => Cube::intersect(ray),
            Surface::UnitCylinder => Cylinder::intersect(ray),
        }
    }

    pub(crate) fn normal_at(&self, object_point: Point) -> Vector {
        match self {
            Surface::UnitSphere => Sphere::normal_at(object_point),
            Surface::PlaneXZ => Plane::normal_at(object_point),
            Surface::UnitCube => Cube::normal_at(object_point),
            Surface::UnitCylinder => Cylinder::normal_at(object_point),
        }
    }
}
