use crate::intersection::{Intersect, Intersection, Intersections};
use crate::lighting::Material;
use crate::rays::Ray;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::point::Point;
use math::tuple::vector::normal::Normal;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: String,
    pub material: Material,
    object_to_world_transform: Matrix4x4,
    world_to_object_transform: Matrix4x4,
}

impl Sphere {
    pub fn normal_at(&self, point: Point) -> Normal {
        let object_point: Point = (self.world_to_object_transform * point).try_into().unwrap();
        let object_normal = object_point - Point::origin();
        let world_normal = self.world_to_object_transform.transpose() * object_normal;
        world_normal.force_vector().normalize()
    }
}

impl Sphere {
    pub fn new_transformed(transform: Matrix4x4) -> Self {
        Self {
            id: format!("{}", Uuid::new_v4()),
            material: Material::default(),
            object_to_world_transform: transform,
            world_to_object_transform: transform.invert().expect("inverse transform failure"),
        }
    }

    pub fn new() -> Self {
        Self::new_transformed(Matrix4x4::identity())
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: Ray) -> Intersections {
        // Convert world ray into object space
        let ray = self.world_to_object_transform * ray;

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

#[cfg(test)]
mod intersection_of_transformed_sphere_tests {
    use super::*;
    use crate::ray;

    #[test]
    fn intersect_scaled_sphere() {
        let sphere = Sphere::new_transformed(Matrix4x4::scale(2., 2., 2.));
        let ray = ray!((0., 0., -5.), (0., 0., 1.));
        let intersections = sphere.intersect(ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 3.);
        assert_eq!(intersections[1].t, 7.);
    }

    #[test]
    fn intersect_translated_sphere() {
        let sphere = Sphere::new_transformed(Matrix4x4::translation(5., 0., 0.));
        let ray = ray!((0., 0., -5.), (0., 0., 1.));
        let intersections = sphere.intersect(ray);
        assert_eq!(intersections.len(), 0);
    }
}

#[cfg(test)]
mod normal_tests {
    use super::*;
    use math::tuple::vector::Vector;
    use math::{point, vector};
    use std::f32::consts::PI;
    use std::ops::Deref;

    #[test]
    fn normal_of_translated_sphere() {
        let sphere = Sphere::new_transformed(Matrix4x4::translation(0., 1., 0.));
        assert_eq!(
            &vector!(0., 0.7071068, -0.70710677),
            sphere.normal_at(point!(0., 1.70711, -0.70711)).deref()
        );
    }

    #[test]
    fn normal_of_transformed_sphere() {
        let sphere = Sphere::new_transformed(Matrix4x4::scale(1., 0.5, 1.).pre_rotation_z(PI / 5.));
        assert_eq!(
            &vector!(-2.0444226e-8, 0.97014254, -0.24253564),
            sphere
                .normal_at(point!(0., 2.0_f32.sqrt() / 2., -2.0_f32.sqrt() / 2.))
                .deref()
        );
    }
}
