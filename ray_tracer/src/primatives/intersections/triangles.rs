use crate::intersection::UV;
use crate::primatives::triangle::{Triangle, TriangleNormal};
use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;

impl Triangle {
    pub(crate) fn intersect(&self, ray: &Ray) -> Option<(UV, f32)> {
        let dir_cross_e2 = ray.direction.cross(self.e2);
        let det = self.e1.dot(&dir_cross_e2);
        if det.abs() < Self::TRIANGLE_EPSILON {
            // Parallel
            return None;
        };

        let f = det.recip();
        let p1 = self.vertices[0];
        let p1_to_origin = ray.origin - p1;
        let u = f * p1_to_origin.dot(&dir_cross_e2);

        if u < 0. || u > 1. {
            // Miss edge p1-p3
            return None;
        }

        let origin_cross_e1 = p1_to_origin.cross(self.e1);
        let v = f * ray.direction.dot(&origin_cross_e1);

        if v < 0. || (u + v) > 1. {
            // Miss edge p1-p2 and p2-p3
            return None;
        }

        Some((UV::new(u, v), f * self.e2.dot(&origin_cross_e1)))
    }

    pub(crate) fn normal_at(&self, _object_point: Point, uv: Option<UV>) -> Vector {
        match self.normal {
            TriangleNormal::Uniform(n) => n,
            TriangleNormal::PerVertex(normals) => self.interpolated_normal(normals, uv.unwrap()),
        }
    }

    fn interpolated_normal(&self, normals: [Vector; 3], uv: UV) -> Vector {
        normals[1] * uv.u + normals[2] * uv.v + normals[0] * (1. - uv.u - uv.v)
    }

    const TRIANGLE_EPSILON: f32 = 0.00001;
}

#[cfg(test)]
mod triangle_intersection_tests {
    use crate::intersection::Intersect;
    use crate::primatives::Shape;
    use math::{point, vector};

    use crate::primatives::triangle::Triangle;
    use crate::ray;

    macro_rules! miss {
        ($($name:ident; $point:expr => $direction:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let ray = ray!($point, $direction);
                    let triangle = Shape::new_triangle(
                        Triangle::new([point!(0,1,0), point!(-1,0,0), point!(1,0,0)])
                    ).to_intersectable();
                    let intersections = triangle.intersect(&ray);
                    assert_eq!(intersections.len(), 0);
                    assert!(!triangle.fast_hit(&ray))
                }
            )*
        };
    }

    miss!(
        a_parallel_ray_misses; point!(0, -1, -2) => vector!(0, 1, 0)
        ray_misses_p1_p3_edge; point!(1, 1, -2)  => vector!(0, 0, 1)
        ray_misses_p1_p2_edge; point!(-1, 1, -2) => vector!(0, 0, 1)
        ray_misses_p2_p3_edge; point!(0, -1, -2)  => vector!(0, 0, 1)
    );

    macro_rules! hit {
        ($($name:ident; $point:expr => $direction:expr; expect_t: $expected_t:expr;
           expect_uv: $expected_u:expr, $expected_v:expr;)*) => {
            $(
                #[test]
                fn $name() {
                    let ray = ray!($point, $direction);
                    let triangle = Shape::new_triangle(
                        Triangle::new([point!(0,1,0), point!(-1,0,0), point!(1,0,0)])
                    ).to_intersectable();
                    let intersections = triangle.intersect(&ray);
                    assert_eq!(intersections.len(), 1);
                    let intersection = intersections.get(0).unwrap();
                    assert_eq!(triangle.id, intersection.shape.id);
                    assert_eq!($expected_t, intersection.t);
                    assert!(triangle.fast_hit(&ray));

                    let uv = intersection.uv.unwrap();
                    assert_eq!($expected_u, uv.u);
                    assert_eq!($expected_v, uv.v);
                }
            )*
        };
    }

    hit!(
        strike_1; point!(0, 0.5, -2) => vector!(0, 0, 1); expect_t: 2.; expect_uv: 0.25, 0.25;
        strike_2; point!(-0.2, 0.3, -2) => vector!(0, 0, 1); expect_t: 2.; expect_uv: 0.45, 0.25;
    );
}

#[cfg(test)]
mod smooth_triangle_normal_interpolation_tests {
    use super::*;
    use crate::intersection::Intersect;
    use crate::primatives::Shape;
    use crate::primatives::intersectable_shape::PointUv;
    use crate::ray_first_gen;
    use math::{assert_vector, point, vector};

    #[test]
    fn a_smooth_triangle_uses_uv_to_interpolate_normal() {
        let n1 = vector!(0, 1, 0);
        let n2 = vector!(-1, 0, 0);
        let n3 = vector!(1, 0, 0);
        let triangle = Shape::new_triangle(Triangle::new_smooth(
            [point!(0, 1, 0), point!(-1, 0, 0), point!(1, 0, 0)],
            [n1, n2, n3],
        ))
        .to_intersectable();
        let normal = triangle.normal_at(PointUv::point_with_some_uv(
            Point::origin(),
            UV::new(0.45, 0.25),
        ));

        assert_vector!(vector!(-0.55470, 0.83205, 0.), normal.to_vector());
    }

    #[test]
    fn a_smooth_triangle_uses_uv_to_interpolate_normal_with_ray() {
        let ray = ray_first_gen!(point!(-0.2, 0.3, -2), vector!(0, 0, 1));

        let n1 = vector!(0, 1, 0);
        let n2 = vector!(-1, 0, 0);
        let n3 = vector!(1, 0, 0);
        let triangle = Shape::new_triangle(Triangle::new_smooth(
            [point!(0, 1, 0), point!(-1, 0, 0), point!(1, 0, 0)],
            [n1, n2, n3],
        ))
        .to_intersectable();

        let intersections = triangle.intersect(&ray);
        assert_eq!(1, intersections.len());
        let intersection = &intersections[0];
        let calculations = intersection.to_pre_calculation(ray);
        assert_vector!(
            vector!(-0.55470, 0.83205, 0.),
            calculations.normal.to_vector()
        );
    }
}

//     #[test]
//     fn tangential_intersection() {
//         let triangle = Shape::new_triangle().to_intersectable();
//         let ray = Ray::new(Point::point(0., 1., -5.), Vector::vector(0., 0., 1.));
//         let intersections = triangle.intersect(&ray);
//         assert_eq!(intersections.len(), 1);
//         assert_eq!(intersections[0].t, 5.);
//         assert!(triangle.fast_hit(&ray))
//     }
//
//     #[test]
//     fn miss() {
//         let triangle = Shape::new_triangle().to_intersectable();
//         let ray = Ray::new(Point::point(0., 2., -5.), Vector::vector(0., 0., 1.));
//         let intersections = triangle.intersect(&ray);
//         assert_eq!(intersections.len(), 0);
//         assert!(!triangle.fast_hit(&ray))
//     }
//
//     #[test]
//     fn ray_from_inside_triangle() {
//         let triangle = Shape::new_triangle().to_intersectable();
//         let ray = Ray::new(Point::origin(), Vector::vector(0., 0., 1.));
//         let intersections = triangle.intersect(&ray);
//         assert_eq!(intersections.len(), 2);
//         assert_eq!(intersections[0].t, -1.);
//         assert_eq!(intersections[1].t, 1.);
//         assert!(triangle.fast_hit(&ray))
//     }
//
//     #[test]
//     fn triangle_behind_ray() {
//         let triangle = Shape::new_triangle().to_intersectable();
//         let ray = Ray::new(Point::point(0., 0., 5.), Vector::vector(0., 0., 1.));
//         let intersections = triangle.intersect(&ray);
//         assert_eq!(intersections.len(), 2);
//         assert_eq!(intersections[0].t, -6.);
//         assert_eq!(intersections[1].t, -4.);
//         assert!(triangle.fast_hit(&ray))
//     }
// }
//
// #[cfg(test)]
// mod triangle_multi_intersection_tests {
//     use crate::intersection::{Intersection, Intersections};
//     use crate::primatives::Shape;
//
//     #[test]
//     fn the_hit_when_all_intersections_are_positive() {
//         let triangle1 = Shape::new_triangle().to_intersectable();
//         let triangle2 = Shape::new_triangle().to_intersectable();
//         let intersections = Intersections::new(vec![
//             Intersection::new(1., &triangle1),
//             Intersection::new(2., &triangle2),
//         ]);
//         assert_eq!(&triangle1, intersections.hit().expect("Expected hit").0.shape);
//     }
//
//     #[test]
//     fn the_hit_when_some_intersections_are_negative() {
//         let triangle1 = Shape::new_triangle().to_intersectable();
//         let triangle2 = Shape::new_triangle().to_intersectable();
//         let intersections = Intersections::new(vec![
//             Intersection::new(-1., &triangle1),
//             Intersection::new(1., &triangle2),
//         ]);
//         assert_eq!(&triangle2, intersections.hit().expect("Expected hit").0.shape);
//     }
//
//     #[test]
//     fn the_hit_when_all_intersections_are_negative() {
//         let triangle1 = Shape::new_triangle().to_intersectable();
//         let triangle2 = Shape::new_triangle().to_intersectable();
//         let intersections = Intersections::new(vec![
//             Intersection::new(-2., &triangle1),
//             Intersection::new(-1., &triangle2),
//         ]);
//         assert!(intersections.hit().is_none());
//     }
//
//     #[test]
//     fn the_hit_is_always_the_lowest_non_negative_intersection() {
//         let triangle1 = Shape::new_triangle().to_intersectable();
//         let triangle2 = Shape::new_triangle().to_intersectable();
//         let triangle3 = Shape::new_triangle().to_intersectable();
//         let triangle4 = Shape::new_triangle().to_intersectable();
//         let intersections = Intersections::new(vec![
//             Intersection::new(5., &triangle1),
//             Intersection::new(7., &triangle2),
//             Intersection::new(-3., &triangle3),
//             Intersection::new(2., &triangle4),
//         ]);
//         assert_eq!(&triangle4, intersections.hit().expect("Expected hit").0.shape);
//     }
// }
//
// #[cfg(test)]
// mod triangle_intersection_of_transformed_triangle_tests {
//     use crate::intersection::Intersect;
//     use crate::primatives::Shape;
//     use crate::ray;
//     use math::matrix::matrix_4x4::Matrix4x4;
//
//     #[test]
//     fn intersect_scaled_triangle() {
//         let triangle = Shape::new_triangle_transformed(Matrix4x4::scale(2., 2., 2.)).to_intersectable();
//         let ray = ray!((0., 0., -5.), (0., 0., 1.));
//         let intersections = triangle.intersect(&ray);
//         assert_eq!(intersections.len(), 2);
//         assert_eq!(intersections[0].t, 3.);
//         assert_eq!(intersections[1].t, 7.);
//         assert!(triangle.fast_hit(&ray))
//     }
//
//     #[test]
//     fn miss_translated_triangle() {
//         let triangle =
//             Shape::new_triangle_transformed(Matrix4x4::translation(5., 0., 0.)).to_intersectable();
//         let ray = ray!((0., 0., -5.), (0., 0., 1.));
//         let intersections = triangle.intersect(&ray);
//         assert_eq!(intersections.len(), 0);
//         assert!(!triangle.fast_hit(&ray))
//     }
// }
//
// #[cfg(test)]
// mod triangle_normal_tests {
//     use crate::primatives::Shape;
//     use math::matrix::matrix_4x4::Matrix4x4;
//     use math::{assert_vector, point, radians, vector};
//     use std::f32::consts::PI;
//
//     #[test]
//     fn normal_of_translated_triangle() {
//         let triangle =
//             Shape::new_triangle_transformed(Matrix4x4::translation(0., 1., 0.)).to_intersectable();
//         assert_vector!(
//             vector!(0., 0.7071, -0.7071),
//             triangle.normal_at(point!(0., 1.70711, -0.70711)).to_vector()
//         );
//     }
//
//     #[test]
//     fn normal_of_transformed_triangle() {
//         let triangle = Shape::new_triangle_transformed(
//             Matrix4x4::scale(1., 0.5, 1.).rotation_z(radians!(PI / 5.)),
//         )
//             .to_intersectable();
//         assert_vector!(
//             vector!(0, 0.9701, -0.2425),
//             triangle
//                 .normal_at(point!(0., 2.0_f32.sqrt() / 2., -2.0_f32.sqrt() / 2.))
//                 .to_vector()
//         );
//     }
// }
