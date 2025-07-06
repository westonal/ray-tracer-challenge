use crate::primatives::ShapeId;
use crate::ray;
use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;

pub struct SurfaceHit<'s> {
    /// Id of shape that owns the surface
    pub shape_id: &'s ShapeId,

    /// Point exactly on surface
    pub point: Point,
}

impl<'s> SurfaceHit<'s> {
    /// Create a ray in the new direction sufficiently positioned off the surface in the direction of the [direction].
    /// This replaces both over and under points and is more consistent as it does not use the surface normal.
    pub(crate) fn new_ray(&self, direction: Vector) -> Ray {
        ray!(self.point + direction * Self::EPSILON, direction)
    }
}

impl<'s> SurfaceHit<'s> {
    pub(crate) fn new(id: &'s ShapeId, point: Point) -> SurfaceHit<'s> {
        Self {
            shape_id: id,
            point,
        }
    }

    // TODO, this is quite large
    pub const EPSILON: f32 = 0.0001;
}

#[cfg(test)]
mod surface_hit_new_ray_tests {
    use super::*;
    use math::{point, vector};

    macro_rules! new_ray_tests {
        ($($name:ident=> $point:expr ; $direction:expr; $expected_point:expr)*) => {
            $(
                #[test]
                fn $name(){
                    let id = &ShapeId::default();
                    let surface_hit = SurfaceHit::new(id, $point);

                    let ray = surface_hit.new_ray($direction);
                    assert_eq!($expected_point, ray.origin);
                    assert_eq!($direction, ray.direction);
                }
            )*
        }
    }

    new_ray_tests! {
       new_ray_in_direction_positive_x => point!(1, 2, 3); vector!(1, 0, 0);  point!(1.0001, 2, 3)
       new_ray_in_direction_negative_x => point!(1, 2, 3); vector!(-1, 0, 0); point!(0.9999, 2, 3)
       new_ray_in_direction_positive_y => point!(1, 2, 3); vector!(0, 1, 0);  point!(1, 2.0001, 3)
       new_ray_in_direction_negative_y => point!(1, 2, 3); vector!(0, -1, 0); point!(1, 1.9999, 3)
       new_ray_in_direction_positive_z => point!(1, 2, 3); vector!(0, 0, 1);  point!(1, 2, 3.0001)
       new_ray_in_direction_negative_z => point!(1, 2, 3); vector!(0, 0, -1); point!(1, 2, 2.9999)
    }
}
