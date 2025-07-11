use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;
use math::vector;

pub struct Plane {}

impl Plane {
    pub(crate) fn intersect(ray: Ray) -> Vec<f32> {
        if ray.direction.y.abs() < f32::EPSILON {
            vec![]
        } else {
            vec![-ray.origin.y / ray.direction.y]
        }
    }

    pub(crate) fn normal_at(_object_point: Point) -> Vector {
        vector!(0, 1, 0)
    }
}

#[cfg(test)]
mod plane_normal_tests {
    use crate::primatives::Shape;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::tuple::point::Point;
    use math::{assert_vector, point, radians, vector};
    use std::f32::consts::PI;

    #[test]
    fn plane_normal() {
        let plane = Shape::new_plane().to_intersectable();
        assert_vector!(vector!(0, 1, 0), *plane.normal_at(Point::origin()));
        assert_vector!(vector!(0, 1, 0), *plane.normal_at(point!(1, 2, 3)));
    }

    #[test]
    fn plane_normal_transformed() {
        let plane = Shape::new_plane_transformed(Matrix4x4::rotation_z(radians!(PI / 2.)))
            .to_intersectable();
        assert_eq!(
            vector!(-1, -4.371139e-8, 0),
            *plane.normal_at(Point::origin())
        );
        assert_eq!(
            vector!(-1, -4.371139e-8, 0),
            *plane.normal_at(point!(1, 2, 3))
        );
    }
}

#[cfg(test)]
mod plane_intersection_tests {

    use crate::intersection::Intersect;
    use crate::primatives::Shape;
    use crate::ray;

    #[test]
    fn parallel_ray_misses() {
        let plane = Shape::new_plane().to_intersectable();
        assert!(plane.intersect(ray!((0., 1., 0.), (1., 0., 0.))).is_empty());
    }

    #[test]
    fn coplanar_ray_misses() {
        let plane = Shape::new_plane().to_intersectable();
        assert!(plane.intersect(ray!((0., 0., 0.), (1., 0., 0.))).is_empty());
    }

    #[test]
    fn ray_intersect_from_above() {
        let plane = Shape::new_plane().to_intersectable();
        let intersections = plane.intersect(ray!((0., 1., 0.), (0., -1., 0.)));
        assert_eq!(1, intersections.len());
        assert_eq!(&plane, intersections[0].shape);
        assert_eq!(1., intersections[0].t);
    }

    #[test]
    fn ray_intersect_from_below() {
        let plane = Shape::new_plane().to_intersectable();
        let intersections = plane.intersect(ray!((0., -1., 0.), (0., 1., 0.)));
        assert_eq!(1, intersections.len());
        assert_eq!(&plane, intersections[0].shape);
        assert_eq!(1., intersections[0].t);
    }

    #[test]
    fn ray_intersect_from_above_behind() {
        let plane = Shape::new_plane().to_intersectable();
        let intersections = plane.intersect(ray!((0., 2., 0.), (0., 1., 0.)));
        assert_eq!(1, intersections.len());
        assert_eq!(&plane, intersections[0].shape);
        assert_eq!(-2., intersections[0].t);
    }
}
