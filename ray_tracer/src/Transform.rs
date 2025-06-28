use crate::rays::Ray;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::point::Point;
use math::tuple::vector::Vector;
use math::tuple::vector::normal::Normal;

#[derive(Debug, PartialEq)]
pub struct Transform {
    object_to_world_transform: Matrix4x4,
    world_to_object_transform: Matrix4x4,
}

impl Transform {
    pub(crate) fn object_normal_to_world_normal(&self, object_normal: Vector) -> Normal {
        let world_normal = self.world_to_object_transform.transpose() * object_normal;
        world_normal.force_vector().normalize()
    }
}

impl Transform {
    pub(crate) fn world_ray_to_object_ray(&self, world_ray: Ray) -> Ray {
        self.world_to_object_transform * world_ray
    }
}

impl Transform {
    pub(crate) fn world_point_to_object_point(&self, point: Point) -> Point {
        (self.world_to_object_transform * point).try_into().unwrap()
    }
}

impl Transform {
    pub fn new(object_to_world_matrix: Matrix4x4) -> Self {
        Self {
            object_to_world_transform: object_to_world_matrix,
            world_to_object_transform: object_to_world_matrix
                .invert()
                .expect("inverse transform failure"),
        }
    }
}
