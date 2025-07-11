use math::tuple::point::Point;
use math::tuple::vector::normal::Normal;
use crate::intersection::{Intersect, Intersection, Intersections};
use crate::material::Material;
use crate::primatives::ShapeId;
use crate::primatives::surface::Surface;
use crate::rays::Ray;
use crate::transform::Transform;

#[derive(Debug, PartialEq, Clone)]
pub struct IntersectableShape {
    pub id: ShapeId,
    pub material: Material,
    pub(crate) transform: Transform,
    pub(crate) surface: Surface,
}

impl IntersectableShape {
    pub fn normal_at(&self, point: Point) -> Normal {
        let object_point: Point = self.transform.world_point_to_object_point(point);
        let object_normal = self.surface.normal_at(object_point);
        self.transform.object_normal_to_world_normal(object_normal)
    }
}

impl Intersect for IntersectableShape {
    fn intersect(&self, ray: Ray) -> Intersections {
        // Convert world ray into object space
        let ray = self.transform.world_ray_to_object_ray(ray);
        let result = self
            .surface
            .intersect(ray)
            .iter()
            .map(|f| Intersection::new(*f, self))
            .collect();
        Intersections::new(result)
    }
}