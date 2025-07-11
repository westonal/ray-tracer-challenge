use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;
use math::{max, min, vector};

pub struct Cube {}

impl Cube {
    pub(crate) fn intersect(ray: Ray) -> Vec<f32> {
        let (x_tmin, x_tmax) = Self::check_axis(ray.origin.x, ray.direction.x);
        let (y_tmin, y_tmax) = Self::check_axis(ray.origin.y, ray.direction.y);
        let (z_tmin, z_tmax) = Self::check_axis(ray.origin.z, ray.direction.z);

        let tmin = max!(x_tmin, y_tmin, z_tmin);
        let tmax = min!(x_tmax, y_tmax, z_tmax);

        if tmin > tmax {
            return vec![];
        }

        vec![tmin, tmax]
    }

    fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
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
        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }

    pub(crate) fn normal_at(object_point: Point) -> Vector {
        let abs_x = object_point.x.abs();
        let abs_y = object_point.y.abs();
        let abs_z = object_point.z.abs();
        let max_c = max!(abs_x, abs_y, abs_z);
        if max_c == abs_x {
            vector!(object_point.x, 0, 0)
        } else if max_c == abs_y {
            vector!(0, object_point.y, 0)
        } else {
            vector!(0, 0, object_point.z)
        }
    }
}

#[cfg(test)]
mod cube_intersection_tests {
    use super::*;
    use crate::intersection::Intersect;
    use crate::primatives::Shape;
    use crate::ray;

    fn run_intersection_test(ray: Ray) -> (f32, f32) {
        let cube = Shape::new_cube().to_shape2();
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
    use crate::intersection::Intersect;
    use crate::primatives::Shape;
    use crate::ray;

    macro_rules! intersection_miss_tests {
        ($($name:ident: $ray:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let cube = Shape::new_cube().to_shape2();
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

#[cfg(test)]
mod cube_normal_tests {
    use crate::primatives::Shape;

    use math::{point, vector};

    macro_rules! cube_normal_tests {
    ($($name:ident: $point:expr => $normal:expr)*) => {
        $(
            #[test]
            fn $name(){
                let cube = Shape::new_cube().to_shape2();
                assert_eq!($normal, *cube.normal_at($point));
            }
        )*
        }
    }

    cube_normal_tests! {
        a: point!(1,0.5,-0.8) => vector!(1,0,0)
        b: point!(-1,0.2,0.9) => vector!(-1,0,0)
        c: point!(-0.4,1,-0.1) => vector!(0,1,0)
        d: point!(0.3,-1,-0.7) => vector!(0,-1,0)
        e: point!(-0.6,0.3,1) => vector!(0,0,1)
        f: point!(0.4,0.4,-1) => vector!(0,0,-1)
        g: point!(1,1,1) => vector!(1,0,0)
        h: point!(-1,-1,-1) => vector!(-1,0,0)
    }
}
