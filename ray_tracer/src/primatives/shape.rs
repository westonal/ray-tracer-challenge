use crate::intersection::{Intersect, Intersection, Intersections};
use crate::material::Material;
use crate::primatives::surface::Surface;
use crate::rays::Ray;
use crate::transform::Transform;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::point::Point;
use math::tuple::vector::normal::Normal;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Shape {
    id: String,
    pub material: Material,
    pub(crate) transform: Transform,
    surface: Surface,
}

impl Shape {
    pub(crate) fn new(object_to_world_matrix: Matrix4x4, surface: Surface) -> Self {
        Self {
            id: format!("{}", Uuid::new_v4()),
            material: Material::default(),
            transform: Transform::new(object_to_world_matrix),
            surface,
        }
    }
}

impl Shape {
    pub fn normal_at(&self, point: Point) -> Normal {
        let object_point: Point = self.transform.world_point_to_object_point(point);
        let object_normal = self.surface.normal_at(object_point);
        self.transform.object_normal_to_world_normal(object_normal)
    }
}

impl Intersect for Shape {
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
