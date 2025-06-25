use crate::intersection::Intersection;
use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::normal::Normal;
use std::ops::Deref;

pub struct PreCalculations<'s> {
    intersection: &'s Intersection<'s>,
    pub point: Point,
    pub eye: Normal,
    pub normal: Normal,
    inside: bool,
}

impl<'s> Intersection<'s> {
    pub fn to_pre_calculation(&'s self, ray: Ray) -> PreCalculations<'s> {
        let point = ray.position(self.t);
        let normal = self.sphere.normal_at(point);
        let inside = normal.dot(&ray.direction) >= 0.;
        let normal = if inside { -normal } else { normal };
        PreCalculations {
            intersection: self,
            point,
            eye: (-ray.direction).normalize(),
            normal,
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
use super::*;
    use crate::intersection::Intersection;
    use crate::primatives::sphere::Sphere;
    use math::{point, vector};
    use crate::ray;

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let ray = ray!((0., 0., -5.), (0., 0., 1.));
        let sphere = Sphere::new();
        let intersection = Intersection::new(4., &sphere);
        let pre_calculations = intersection.to_pre_calculation(ray);
        assert_eq!(4., pre_calculations.t);
        assert_eq!(&sphere, pre_calculations.sphere);
        assert_eq!(point!(0, 0, -1), pre_calculations.point);
        assert_eq!(vector!(0, 0, -1), pre_calculations.eye.clone_vector());
        assert_eq!(vector!(0, 0, -1), pre_calculations.normal.clone_vector());
        assert!(!pre_calculations.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let ray = ray!((0., 0., 0.), (0., 0., 1.));
        let sphere = Sphere::new();
        let intersection = Intersection::new(1., &sphere);
        let pre_calculations = intersection.to_pre_calculation(ray);
        assert_eq!(1., pre_calculations.t);
        assert_eq!(&sphere, pre_calculations.sphere);
        assert_eq!(point!(0, 0, 1), pre_calculations.point);
        assert_eq!(vector!(0, 0, -1), pre_calculations.eye.clone_vector());
        assert_eq!(vector!(0, 0, -1), pre_calculations.normal.clone_vector());
        assert!(pre_calculations.inside);
    }
}
