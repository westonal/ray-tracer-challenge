use crate::primatives::Shape;
use crate::primatives::surface::Surface;
use crate::primatives::surface::Surface::UnitCube;
use crate::rays::Ray;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::point::Point;
use math::tuple::vector::Vector;
use math::vector;
use std::cmp::max;

impl Shape {
    pub fn new_cube_transformed(transform: Matrix4x4) -> Self {
        Self::new(transform, UnitCube)
    }

    pub fn new_cube() -> Self {
        Self::new_cube_transformed(Matrix4x4::identity())
    }
}
impl Surface {
    pub(crate) fn cube_intersect(&self, ray: Ray) -> Vec<f32> {
        let (x_tmin, x_tmax) = self.check_axis(ray.origin.x, ray.direction.x);
        let (y_tmin, y_tmax) = self.check_axis(ray.origin.y, ray.direction.y);
        let (z_tmin, z_tmax) = self.check_axis(ray.origin.z, ray.direction.z);

        let tmin = x_tmin.max(y_tmin.max(z_tmin));
        let tmax = x_tmax.min(y_tmax.min(z_tmax));

        if tmin > tmax {
            return vec![]
        }

        vec![tmin, tmax]
    }

    fn check_axis(&self, origin: f32, direction: f32) -> (f32, f32) {
        let tmin_numerator = -1. - origin;
        let tmax_numerator = 1. - origin;
        let (tmin, tmax) = if direction.abs() >= f32::EPSILON {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (
                tmin_numerator * f32::INFINITY,
                tmax_numerator * f32::INFINITY,
            )
        };
        if (tmin > tmax){
            (tmax, tmin)
        }else{
            (tmin, tmax)
        }
    }

    pub(crate) fn cube_normal_at(&self, _object_point: Point) -> Vector {
        vector!(0, 1, 0)
    }
}

#[cfg(test)]
mod cube_normal_tests {
    use crate::primatives::Shape;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::tuple::point::Point;
    use math::{point, radians, vector};
    use std::f32::consts::PI;

    #[test]
    fn cube_normal() {
        let cube = Shape::new_cube();
        assert_eq!(vector!(0, 1, 0), *cube.normal_at(Point::origin()));
        assert_eq!(vector!(0, 1, 0), *cube.normal_at(point!(1, 2, 3)));
    }

    #[test]
    fn cube_normal_transformed() {
        let cube = Shape::new_cube_transformed(Matrix4x4::rotation_z(radians!(PI / 2.)));
        assert_eq!(
            vector!(-1, -4.371139e-8, 0),
            *cube.normal_at(Point::origin())
        );
        assert_eq!(
            vector!(-1, -4.371139e-8, 0),
            *cube.normal_at(point!(1, 2, 3))
        );
    }
}

#[cfg(test)]
mod cube_intersection_tests {
    use super::*;
    use crate::intersection::Intersect;
    use crate::ray;

    fn run_intersection_test(ray: Ray) -> (f32, f32) {
        let cube = Shape::new_cube();
        let intersections = cube.intersect(ray);
        assert_eq!(2, intersections.len());
        (
            intersections.get(0).unwrap().t,
            intersections.get(1).unwrap().t,
        )
    }

    macro_rules! intersection_tests {
    ($($name:ident: $ray:expr => $value:expr)*) => {
    $(
        #[test]
        fn $name(){
            let (expected_n1,expected_n2) = $value;
            let (actual_n1,actual_n2) = run_intersection_test($ray);
            assert_eq!(expected_n1, actual_n1, "n1");
            assert_eq!(expected_n2, actual_n2, "n2");
        }
    )*
    }
        }

    intersection_tests! {
        plus_x: ray!((5.,0.5,0.),(-1.,0.,0.))=>(4.,6.)
        minus_x: ray!((-5.,0.5,0.),(1.,0.,0.))=>(4.,6.)
        plus_y: ray!((0.5,5.,0.),(0.,-1.,0.))=>(4.,6.)
        minus_y: ray!((0.5,-5.,0.),(0.,1.,0.))=>(4.,6.)
        plus_z: ray!((0.5,0.,5.),(0.,0.,-1.))=>(4.,6.)
        minus_z: ray!((0.5,0.,-5.),(0.,0.,1.))=>(4.,6.)
        inside: ray!((0.,0.5,0.),(0.,0.,1.))=>(-1.,1.)
    }
}

#[cfg(test)]
mod cube_intersection_missing_tests {
    use super::*;
    use crate::intersection::Intersect;
    use crate::ray;

    macro_rules! intersection_miss_tests {
    ($($name:ident: $ray:expr)*) => {
    $(
        #[test]
        fn $name(){
            let cube = Shape::new_cube();
            let intersections = cube.intersect($ray);
            assert_eq!(0, intersections.len());
        }
    )*
    }
        }

    intersection_miss_tests! {
        from_x: ray!((-2.,0.,0.), (0.2673,0.5345,0.8018))
        from_y: ray!((0.,-2.,0.), (0.8018, 0.2673,0.5345))
        from_z: ray!((0.,0.,-2.), (0.5345,0.8018,0.2673))
        from_x_z: ray!((2.,0.,2.), (0.,0.,-1.))
        from_y_z: ray!((0.,2.,2.), (0.,-1.,0.))
        from_x_y: ray!((2.,2.,0.), (-1.,0.,0.))
    }
}
