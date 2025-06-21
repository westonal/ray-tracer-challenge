use crate::intersection::{Intersect, Intersection, Intersections};
use crate::rays::Ray;
use math::tuple::point::Point;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: String,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: format!("{}", Uuid::new_v4()),
        }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: Ray) -> Intersections {
        let sphere_to_ray = ray.origin - Point::origin();
        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return Intersections(vec![]);
        }
        let mut result: Vec<Intersection> = Vec::with_capacity(2);
        let a2 = 2. * a;
        if discriminant == 0. {
            result.push(Intersection::new(-b / a2, self));
        } else {
            let discriminant_sqrt = discriminant.sqrt();
            result.push(Intersection::new((-b - discriminant_sqrt) / a2, self));
            result.push(Intersection::new((-b + discriminant_sqrt) / a2, self));
        }
        Intersections(result)
    }
}

#[cfg(test)]
mod sphere_intersection_tests {
    use super::*;
    use crate::rays::Ray;
    use math::tuple::point::Point;
    use math::tuple::vector::Vector;

    #[test]
    fn intersect() {
        let sphere = Sphere::new();
        let ray = Ray::new(Point::point(0., 0., -5.), Vector::vector(0., 0., 1.));
        let intersections = sphere.intersect(ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 4.);
        assert_eq!(intersections[1].t, 6.);
    }

    #[test]
    fn tangential_intersection() {
        let sphere = Sphere::new();
        let ray = Ray::new(Point::point(0., 1., -5.), Vector::vector(0., 0., 1.));
        let intersections = sphere.intersect(ray);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 5.);
    }

    #[test]
    fn miss() {
        let sphere = Sphere::new();
        let ray = Ray::new(Point::point(0., 2., -5.), Vector::vector(0., 0., 1.));
        let intersections = sphere.intersect(ray);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_from_inside_sphere() {
        let sphere = Sphere::new();
        let ray = Ray::new(Point::origin(), Vector::vector(0., 0., 1.));
        let intersections = sphere.intersect(ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -1.);
        assert_eq!(intersections[1].t, 1.);
    }

    #[test]
    fn sphere_behind_ray() {
        let sphere = Sphere::new();
        let ray = Ray::new(Point::point(0., 0., 5.), Vector::vector(0., 0., 1.));
        let intersections = sphere.intersect(ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -6.);
        assert_eq!(intersections[1].t, -4.);
    }
}

#[cfg(test)]
mod multi_intersection_tests {
    use super::*;

    #[test]
    fn the_hit_when_all_intersections_are_positive() {
        let sphere1 = Sphere::new();
        let sphere2 = Sphere::new();
        let intersections = Intersections(vec![
            Intersection::new(1., &sphere1),
            Intersection::new(2., &sphere2),
        ]);
        assert_eq!(&sphere1, intersections.hit().expect("Expected hit").sphere);
    }

    #[test]
    fn the_hit_when_some_intersections_are_negative() {
        let sphere1 = Sphere::new();
        let sphere2 = Sphere::new();
        let intersections = Intersections(vec![
            Intersection::new(-1., &sphere1),
            Intersection::new(1., &sphere2),
        ]);
        assert_eq!(&sphere2, intersections.hit().expect("Expected hit").sphere);
    }

    #[test]
    fn the_hit_when_all_intersections_are_negative() {
        let sphere1 = Sphere::new();
        let sphere2 = Sphere::new();
        let intersections = Intersections(vec![
            Intersection::new(-2., &sphere1),
            Intersection::new(-1., &sphere2),
        ]);
        assert!(intersections.hit().is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let sphere1 = Sphere::new();
        let sphere2 = Sphere::new();
        let sphere3 = Sphere::new();
        let sphere4 = Sphere::new();
        let intersections = Intersections(vec![
            Intersection::new(5., &sphere1),
            Intersection::new(7., &sphere2),
            Intersection::new(-3., &sphere3),
            Intersection::new(2., &sphere4),
        ]);
        assert_eq!(&sphere4, intersections.hit().expect("Expected hit").sphere);
    }
}
