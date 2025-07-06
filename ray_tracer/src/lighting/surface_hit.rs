use crate::primatives::ShapeId;
use math::tuple::point::Point;
use math::tuple::vector::normal::Normal;

pub struct SurfaceHit<'s> {
    /// Id of shape that owns the surface
    pub shape_id: &'s ShapeId,

    /// Point exactly on surface
    pub point: Point,

    /// Point above surface
    pub over_point: Point,

    /// Point under surface
    pub under_point: Point,
}

impl<'s> SurfaceHit<'s> {
    pub(crate) fn new(id: &'s ShapeId, point: Point, normal: &Normal) -> SurfaceHit<'s> {
        let small_adjustment_for_under_over_points = normal.clone_vector() * Self::EPSILON;
        Self {
            shape_id: id,
            point,
            over_point: point + small_adjustment_for_under_over_points,
            under_point: point - small_adjustment_for_under_over_points,
        }
    }

    // TODO, this is quite large
    pub const EPSILON: f32 = 0.0001;
}


#[cfg(test)]
mod surface_hit_point_tests {
    use super::*;
    use crate::ray_first_gen;
    use math::matrix::matrix_4x4::Matrix4x4;
    use crate::intersection::Intersection;
    use crate::primatives::Shape;

    #[test]
    fn the_hit_should_offset_the_point_over() {
        let shape = Shape::new_sphere_transformed(Matrix4x4::translation(0., 0., 1.));
        let i = Intersection::new(5., &shape);
        let calcs = i.to_pre_calculation(ray_first_gen!((0., 0., -5.), (0., 0., 1.)));
        assert!(calcs.surface_hit.over_point.z < -SurfaceHit::EPSILON / 2.);
        assert!(calcs.surface_hit.point.z > calcs.surface_hit.over_point.z);
    }

    #[test]
    fn the_hit_should_offset_the_point_under() {
        let shape = Shape::new_sphere_transformed(Matrix4x4::translation(0., 0., 1.));
        let i = Intersection::new(5., &shape);
        let calcs = i.to_pre_calculation(ray_first_gen!((0., 0., -5.), (0., 0., 1.)));
        assert!(calcs.surface_hit.under_point.z > SurfaceHit::EPSILON / 2.);
        assert!(calcs.surface_hit.point.z < calcs.surface_hit.under_point.z);
    }
}