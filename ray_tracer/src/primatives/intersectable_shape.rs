use crate::intersection::{Intersect, Intersection, Intersections, UV};
use crate::material::Material;
use crate::primatives::ShapeId;
use crate::primatives::surface::Surface;
use crate::rays::Ray;
use crate::transform::Transform;
use math::tuple::point::Point;
use math::tuple::vector::normal::Normal;

#[derive(Debug, PartialEq, Clone)]
pub struct IntersectableShape {
    pub id: ShapeId,
    pub material: Material,
    pub(crate) transform: Transform,
    pub(crate) surface: Surface,
}



impl IntersectableShape {
    pub fn normal_at_with_uv(&self, point: Point, uv: Option<UV>) -> Normal {
        let object_point: Point = self.transform.world_point_to_object_point(point);
        let object_normal = self.surface.normal_at(object_point, uv);
        self.transform.object_normal_to_world_normal(object_normal)
    }

    #[cfg(test)]
    pub fn normal_at(&self, point: Point) -> Normal {
        self.normal_at_with_uv(point, None)
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
