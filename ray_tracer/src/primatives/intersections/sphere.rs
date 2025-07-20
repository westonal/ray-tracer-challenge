use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;

pub struct Sphere {}

impl Sphere {
    pub(crate) fn intersect(ray: &Ray) -> Vec<f32> {
        let sphere_to_ray = ray.origin - Point::origin();
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return Default::default();
        }
        let mut result = Vec::with_capacity(2);
        let a2 = 2. * a;
        if discriminant == 0. {
            result.push(-b / a2);
        } else {
            let discriminant_sqrt = discriminant.sqrt();
            result.push((-b - discriminant_sqrt) / a2);
            result.push((-b + discriminant_sqrt) / a2);
        }
        result
    }

    pub(crate) fn normal_at(object_point: Point) -> Vector {
        object_point - Point::origin()
    }
}

#[cfg(test)]
mod sphere_intersection_tests {
    use crate::intersection::Intersect;

    use crate::rays::Ray;
    use crate::sphere;
    use math::tuple::point::Point;
    use math::tuple::vector::Vector;

    #[test]
    fn intersect() {
        let sphere = sphere!().to_intersectable();
        let ray = Ray::new(Point::point(0., 0., -5.), Vector::vector(0., 0., 1.));
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 4.);
        assert_eq!(intersections[1].t, 6.);
        assert!(sphere.fast_hit(&ray))
    }

    #[test]
    fn tangential_intersection() {
        let sphere = sphere!().to_intersectable();
        let ray = Ray::new(Point::point(0., 1., -5.), Vector::vector(0., 0., 1.));
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 5.);
        assert!(sphere.fast_hit(&ray))
    }

    #[test]
    fn miss() {
        let sphere = sphere!().to_intersectable();
        let ray = Ray::new(Point::point(0., 2., -5.), Vector::vector(0., 0., 1.));
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 0);
        assert!(!sphere.fast_hit(&ray))
    }

    #[test]
    fn ray_from_inside_sphere() {
        let sphere = sphere!().to_intersectable();
        let ray = Ray::new(Point::origin(), Vector::vector(0., 0., 1.));
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -1.);
        assert_eq!(intersections[1].t, 1.);
        assert!(sphere.fast_hit(&ray))
    }

    #[test]
    fn sphere_behind_ray() {
        let sphere = sphere!().to_intersectable();
        let ray = Ray::new(Point::point(0., 0., 5.), Vector::vector(0., 0., 1.));
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -6.);
        assert_eq!(intersections[1].t, -4.);
        assert!(sphere.fast_hit(&ray))
    }
}

#[cfg(test)]
mod sphere_multi_intersection_tests {
    use crate::intersection::{Intersection, Intersections};

    use crate::sphere;

    #[test]
    fn the_hit_when_all_intersections_are_positive() {
        let sphere1 = sphere!().to_intersectable();
        let sphere2 = sphere!().to_intersectable();
        let intersections = Intersections::new(vec![
            Intersection::new(1., &sphere1),
            Intersection::new(2., &sphere2),
        ]);
        assert_eq!(&sphere1, intersections.hit().expect("Expected hit").0.shape);
    }

    #[test]
    fn the_hit_when_some_intersections_are_negative() {
        let sphere1 = sphere!().to_intersectable();
        let sphere2 = sphere!().to_intersectable();
        let intersections = Intersections::new(vec![
            Intersection::new(-1., &sphere1),
            Intersection::new(1., &sphere2),
        ]);
        assert_eq!(&sphere2, intersections.hit().expect("Expected hit").0.shape);
    }

    #[test]
    fn the_hit_when_all_intersections_are_negative() {
        let sphere1 = sphere!().to_intersectable();
        let sphere2 = sphere!().to_intersectable();
        let intersections = Intersections::new(vec![
            Intersection::new(-2., &sphere1),
            Intersection::new(-1., &sphere2),
        ]);
        assert!(intersections.hit().is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let sphere1 = sphere!().to_intersectable();
        let sphere2 = sphere!().to_intersectable();
        let sphere3 = sphere!().to_intersectable();
        let sphere4 = sphere!().to_intersectable();
        let intersections = Intersections::new(vec![
            Intersection::new(5., &sphere1),
            Intersection::new(7., &sphere2),
            Intersection::new(-3., &sphere3),
            Intersection::new(2., &sphere4),
        ]);
        assert_eq!(&sphere4, intersections.hit().expect("Expected hit").0.shape);
    }
}

#[cfg(test)]
mod sphere_intersection_of_transformed_sphere_tests {
    use crate::intersection::Intersect;

    use crate::{ray, sphere};
    use math::matrix::matrix_4x4::Matrix4x4;

    #[test]
    fn intersect_scaled_sphere() {
        let sphere = sphere!(matrix: Matrix4x4::scale(2., 2., 2.)).to_intersectable();
        let ray = ray!((0., 0., -5.), (0., 0., 1.));
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 3.);
        assert_eq!(intersections[1].t, 7.);
        assert!(sphere.fast_hit(&ray))
    }

    #[test]
    fn miss_translated_sphere() {
        let sphere = sphere!(matrix: Matrix4x4::translation(5., 0., 0.)).to_intersectable();
        let ray = ray!((0., 0., -5.), (0., 0., 1.));
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 0);
        assert!(!sphere.fast_hit(&ray))
    }
}

#[cfg(test)]
mod sphere_normal_tests {

    use crate::sphere;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::{assert_vector, point, radians, vector};
    use std::f32::consts::PI;

    #[test]
    fn normal_of_translated_sphere() {
        let transform = Matrix4x4::translation(0., 1., 0.);
        let sphere = sphere!(matrix: transform).to_intersectable();
        assert_vector!(
            vector!(0., 0.7071, -0.7071),
            sphere
                .normal_at(point!(0., 1.70711, -0.70711).into())
                .to_vector()
        );
    }

    #[test]
    fn normal_of_transformed_sphere() {
        let transform = Matrix4x4::scale(1., 0.5, 1.).pre_rotation_z(radians!(PI / 5.));
        let sphere = sphere!(matrix: transform).to_intersectable();
        assert_vector!(
            vector!(0, 0.9701, -0.2425),
            sphere
                .normal_at(point!(0., 2.0_f32.sqrt() / 2., -2.0_f32.sqrt() / 2.).into())
                .to_vector()
        );
    }
}
