use crate::intersection::{UV, UVt};
use crate::primatives::intersections::Sphere;
use crate::primatives::intersections::{Cube, Cylinder};
use crate::primatives::intersections::{CylinderCapStyle, Plane};
use crate::primatives::triangle::Triangle;
use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Surface {
    /// A sphere with center at origin and radius 1
    UnitSphere,

    /// AABB Axis-Aligned Bounding Box
    UnitCube,

    /// The Plane Y=0
    PlaneXZ,

    /// A cylinder with center at origin and radius 1, infinite length
    UnitCylinder(CylinderCapStyle),

    SingleTriangle(Triangle),
}

impl Surface {
    pub(crate) fn fast_hit(&self, ray: &Ray) -> bool {
        match self {
            Surface::UnitSphere => !Sphere::intersect(ray).is_empty(),
            Surface::PlaneXZ => !Plane::intersect(ray).is_empty(),
            Surface::UnitCube => Cube::fast_hit(ray),
            Surface::UnitCylinder(style) => !Cylinder::intersect(ray, style).is_empty(),
            Surface::SingleTriangle(triangle) => triangle.intersect(ray).is_some(),
        }
    }

    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<UVt> {
        match self {
            Surface::UnitSphere => Sphere::intersect(ray)
                .into_iter()
                .map(|t| UVt::just_t(t))
                .collect(),
            Surface::PlaneXZ => Plane::intersect(ray)
                .into_iter()
                .map(|t| UVt::just_t(t))
                .collect(),
            Surface::UnitCube => Cube::intersect(ray)
                .into_iter()
                .map(|t| UVt::just_t(t))
                .collect(),
            Surface::UnitCylinder(style) => Cylinder::intersect(ray, style)
                .into_iter()
                .map(|t| UVt::just_t(t))
                .collect(),
            Surface::SingleTriangle(triangle) => triangle
                .intersect(ray)
                .into_iter()
                .map(|(uv, t)| UVt::new(uv, t))
                .collect(),
        }
    }

    pub(crate) fn normal_at(&self, object_point: Point, uv: Option<UV>) -> Vector {
        match self {
            Surface::UnitSphere => Sphere::normal_at(object_point),
            Surface::PlaneXZ => Plane::normal_at(object_point),
            Surface::UnitCube => Cube::normal_at(object_point),
            Surface::UnitCylinder(style) => Cylinder::normal_at(object_point, style),
            &Surface::SingleTriangle(triangle) => triangle.normal_at(object_point, uv),
        }
    }
}
