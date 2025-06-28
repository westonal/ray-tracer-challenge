use crate::intersection::{Intersect, Intersection, Intersections};
use crate::lighting::Material;
use crate::primatives::surface::Surface;
use crate::rays::Ray;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::point::Point;
use math::tuple::vector::normal::Normal;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Shape {
    id: String,
    pub material: Material,
    object_to_world_transform: Matrix4x4,
    world_to_object_transform: Matrix4x4,
    surface: Surface,
}

impl Shape {
    pub fn new(transform: Matrix4x4, surface: Surface) -> Self {
        Self {
            id: format!("{}", Uuid::new_v4()),
            material: Material::default(),
            object_to_world_transform: transform,
            world_to_object_transform: transform.invert().expect("inverse transform failure"),
            surface,
        }
    }
}

impl Shape {
    pub fn normal_at(&self, point: Point) -> Normal {
        let object_point: Point = (self.world_to_object_transform * point).try_into().unwrap();
        let object_normal = self.surface.normal_at(object_point);
        let world_normal = self.world_to_object_transform.transpose() * object_normal;
        world_normal.force_vector().normalize()
    }
}

impl Intersect for Shape {
    fn intersect(&self, ray: Ray) -> Intersections {
        // Convert world ray into object space
        let ray = self.world_to_object_transform * ray;
        let result = self
            .surface
            .intersect(ray)
            .iter()
            .map(|f| Intersection::new(*f, self))
            .collect();
        Intersections::new(result)
    }
}
