use crate::intersection::Intersection;
use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;
use math::tuple::vector::normal::Normal;
use std::ops::Deref;

pub struct PreCalculations<'s> {
    intersection: &'s Intersection<'s>,
    pub point: Point,
    pub over_point: Point,
    pub eye: Normal,
    pub normal: Normal,
    pub reflection: Vector,
    inside: bool,
}

impl<'s> Intersection<'s> {
    pub fn to_pre_calculation(&'s self, ray: Ray) -> PreCalculations<'s> {
        let point = ray.position(self.t);
        let normal = self.shape.normal_at(point);
        let inside = normal.dot(&ray.direction) >= 0.;
        let normal = if inside { -normal } else { normal };
        let normal_as_vector = normal.clone_vector();
        PreCalculations {
            intersection: self,
            point,
            over_point: point + normal_as_vector * Intersection::EPSILON,
            eye: (-ray.direction).normalize(),
            normal,
            reflection: ray.direction.reflect(normal_as_vector),
            inside,
        }
    }
}

impl<'s> Deref for PreCalculations<'s> {
    type Target = Intersection<'s>;

    fn deref(&self) -> &Self::Target {
        self.intersection
    }
}

#[cfg(test)]
mod precalculation_tests {
    use crate::intersection::Intersection;
    use crate::primatives::Shape;
    use crate::ray;
    use math::{point, vector};

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let ray = ray!((0., 0., -5.), (0., 0., 1.));
        let sphere = Shape::new_sphere();
        let intersection = Intersection::new(4., &sphere);
        let pre_calculations = intersection.to_pre_calculation(ray);
        assert_eq!(4., pre_calculations.t);
        assert_eq!(&sphere, pre_calculations.shape);
        assert_eq!(point!(0, 0, -1), pre_calculations.point);
        assert_eq!(vector!(0, 0, -1), pre_calculations.eye.clone_vector());
        assert_eq!(vector!(0, 0, -1), pre_calculations.normal.clone_vector());
        assert!(!pre_calculations.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let ray = ray!((0., 0., 0.), (0., 0., 1.));
        let sphere = Shape::new_sphere();
        let intersection = Intersection::new(1., &sphere);
        let pre_calculations = intersection.to_pre_calculation(ray);
        assert_eq!(1., pre_calculations.t);
        assert_eq!(&sphere, pre_calculations.shape);
        assert_eq!(point!(0, 0, 1), pre_calculations.point);
        assert_eq!(vector!(0, 0, -1), pre_calculations.eye.clone_vector());
        assert_eq!(vector!(0, 0, -1), pre_calculations.normal.clone_vector());
        assert!(pre_calculations.inside);
    }
}

#[cfg(test)]
mod reflection_pre_calc_tests {
    use super::*;
    use crate::primatives::Shape;
    use crate::ray;
    use math::vector;

    #[test]
    fn precompute_the_reflection_vector() {
        let plane = Shape::new_plane();
        let ray = ray!(
            (0., 1., -1.),
            (0., -(2.0_f32.sqrt() / 2.), 2.0_f32.sqrt() / 2.)
        );
        let i = Intersection::new(2.0_f32.sqrt(), &plane);
        let calculations = i.to_pre_calculation(ray);
        assert_eq!(
            vector!(0, 2.0_f32.sqrt() / 2., 2.0_f32.sqrt() / 2.),
            calculations.reflection
        );
    }
}
