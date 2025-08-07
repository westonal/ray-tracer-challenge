use crate::intersection::{Intersect, Intersection, Intersections, UV};
use crate::material::Material;
use crate::primatives::ShapeId;
use crate::primatives::surface::Surface;
use crate::rays::Ray;
use crate::transform::Transform;
use math::tuple::point::Point;
use math::tuple::vector::normal::Normal;
use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Clone)]
pub struct IntersectableShape {
    pub id: ShapeId,
    pub material: Material,
    pub transform: Transform,
    pub surface: Surface,
}

impl Debug for IntersectableShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.surface, self.id)
    }
}

pub struct PointUv {
    point: Point,
    uv: Option<UV>,
}

impl PointUv {
    pub fn new(point: Point, uv: Option<UV>) -> Self {
        Self { point, uv }
    }

    #[cfg(test)]
    pub fn point_no_uv(point: Point) -> Self {
        Self::new(point, None)
    }

    pub fn point_with_some_uv(point: Point, uv: UV) -> Self {
        Self::new(point, Some(uv))
    }
}

#[cfg(test)]
impl From<Point> for PointUv {
    fn from(value: Point) -> Self {
        PointUv::point_no_uv(value)
    }
}

impl IntersectableShape {
    pub fn normal_at(&self, point_uv: PointUv) -> Normal {
        let object_point: Point = self.transform.world_point_to_object_point(point_uv.point);
        let object_normal = self.surface.normal_at(object_point, point_uv.uv);
        self.transform.object_normal_to_world_normal(object_normal)
    }
}

impl Intersect for IntersectableShape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        // Convert world ray into object space
        let ray = self.transform.world_ray_to_object_ray(ray);
        let result = self
            .surface
            .intersect(&ray)
            .iter()
            .map(|f| Intersection::new_fuv(*f, self))
            .collect();
        Intersections::new(result)
    }

    fn fast_hit(&self, ray: &Ray) -> bool {
        let ray = self.transform.world_ray_to_object_ray(ray);
        self.surface.fast_hit(&ray)
    }
}
