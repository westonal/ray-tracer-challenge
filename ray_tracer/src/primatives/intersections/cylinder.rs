use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;
use math::vector;

pub struct Cylinder {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CylinderCapStyle {
    Open,
    Closed,
}

impl Cylinder {
    pub(crate) fn intersect(ray: &Ray, cylinder_cap_style: &CylinderCapStyle) -> Vec<f32> {
        let direction = vector!(ray.direction.x, 0., ray.direction.z);

        let mut result = Vec::with_capacity(2);
        let limit = match cylinder_cap_style {
            CylinderCapStyle::Open => 1.,
            CylinderCapStyle::Closed => {
                let t = (-1. - ray.origin.y) / ray.direction.y;
                if Self::check_cap(&ray.position(t)) {
                    result.push(t);
                }
                let t = (1. - ray.origin.y) / ray.direction.y;
                if Self::check_cap(&ray.position(t)) {
                    result.push(t);
                }
                // Small EPSILON to ensure no rays escape caps
                1. + Self::CAP_EPSILON
            }
        };

        let a = direction.dot(&direction);
        if a == 0. {
            return result;
        }

        let cylinder_to_ray = vector!(ray.origin.x, 0., ray.origin.z);
        let b = 2. * direction.dot(&cylinder_to_ray);
        let c = cylinder_to_ray.dot(&cylinder_to_ray) - 1.0;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return result;
        }
        let a2 = 2. * a;
        if discriminant == 0. {
            result.push(-b / a2);
        } else {
            let discriminant_sqrt = discriminant.sqrt();
            let t = (-b - discriminant_sqrt) / a2;
            if Self::near(&ray, t, limit) {
                result.push(t)
            }
            let t = (-b + discriminant_sqrt) / a2;
            if Self::near(&ray, t, limit) {
                result.push(t)
            }
        }
        result
    }

    fn check_cap(point: &Point) -> bool {
        Self::d_squared_xz(point) <= 1.
    }

    fn near(ray: &Ray, t: f32, limit: f32) -> bool {
        let position = ray.position(t);
        position.y.abs() < limit
    }

    fn d_squared_xz(point: &Point) -> f32 {
        point.x * point.x + point.z * point.z
    }

    pub(crate) fn normal_at(object_point: Point, cylinder_cap_style: &CylinderCapStyle) -> Vector {
        if cylinder_cap_style == &CylinderCapStyle::Closed && Self::d_squared_xz(&object_point) < 1.
        {
            if object_point.y >= 1. - Self::CAP_EPSILON {
                return vector!(0, 1., 0);
            }
            if object_point.y <= -1. + Self::CAP_EPSILON {
                return vector!(0, -1., 0);
            }
        }
        vector!(object_point.x, 0., object_point.z)
    }

    const CAP_EPSILON: f32 = 0.00001;
}

#[cfg(test)]
mod cylinder_intersection_miss_tests {
    use crate::intersection::Intersect;

    use crate::ray;

    macro_rules! miss {
        ($($name:ident; $ray:expr)*) => {
            $(
            #[test]
            fn $name() {

                let cylinder = $crate::cylinder_open!().to_intersectable();
                let intersections = cylinder.intersect(&$ray);
                assert_eq!(intersections.len(), 0);
                assert!(!cylinder.fast_hit(&$ray))
            }
            )*
        };
    }

    miss!(
        origin; ray!((0.,0.,0.), (0.,1.,0.))
        up; ray!((1.,0.,0.), (0.,1.,0.))
        b; ray!((0.,0.,-5.), (1.,1.,1.))
    );
}

#[cfg(test)]
mod cylinder_intersection_hit_tests {
    use crate::intersection::Intersect;

    use crate::ray;
    use math::matrix::matrix_4x4::*;

    macro_rules! hit {
        ($($name:ident; $ray:expr => $t:expr)*) => {
            $(
            #[test]
            fn $name() {
                let cylinder = $crate::cylinder_open!(matrix: Matrix4x4::scale(1., 8., 1.)).to_intersectable();
                let intersections = cylinder.intersect(&$ray.normalize());
                assert_eq!(intersections.iter().map(|a|a.t).collect::<Vec<f32>>(), $t);
                assert!(cylinder.fast_hit(&$ray))
            }
            )*
        };
    }

    hit!(
        tangential; ray!((1., 0., -5.),  (0., 0., 1.))  => vec!(5.)
        horizontal; ray!((0., 0., -5.),  (0., 0., 1.))  => vec!(4., 6.)
        angled;     ray!((0.5, 0., -5.), (0.1, 1., 1.)) => vec!(6.808006, 7.0886984)
        inside;     ray!((0., 0., 0.5),  (0., 0., 1.))  => vec!(-1.5, 0.5)
        behind;     ray!((0., 0., 2.),   (0., 0., 1.))  => vec!(-3., -1.)
    );
}

#[cfg(test)]
mod cylinder_normal_tests {

    use math::{point, vector};

    macro_rules! normal {
        ($($name:ident; $point:expr => $expect:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let cylinder = $crate::cylinder_open!().to_intersectable();
                    math::assert_vector!(cylinder.normal_at($point.into()).to_vector(), $expect);
                }
            )*
        };
    }

    normal!(
        a; point!(1, 0, 0) => vector!(1, 0, 0)
        b; point!(0, 5, -1) => vector!(0, 0, -1)
        c; point!(0, -2, 1) => vector!(0, 0, 1)
        d; point!(-1, 1, 0) => vector!(-1, 0, 0)
    );
}

#[cfg(test)]
mod cylinder_truncate_tests {

    use crate::intersection::Intersect;

    use crate::ray;
    use math::matrix::matrix_4x4::*;

    macro_rules! truncate_hits {
        ($($name:ident; $ray:expr => $expect:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let cylinder = $crate::cylinder_open!(matrix: Matrix4x4::translation(0., 1., 0.)
                        .scale(1., 0.5, 1.)
                        .translation(0., 1., 0.)).to_intersectable();
                    let intersections = cylinder.intersect(&$ray.normalize());
                    assert_eq!($expect, intersections.len());
                    if $expect > 0 {
                        assert!(cylinder.fast_hit(&$ray));
                    }else{
                        assert!(!cylinder.fast_hit(&$ray));
                    }
                }
            )*
        };
    }

    truncate_hits!(
        test_1; ray!((0., 1.5, 0.),  (0.1, 1., 0.))  => 0
        test_2; ray!((0., 3., -5.),  (0., 0., 1.))  => 0
        test_3; ray!((0., 0., -5.),  (0., 0., 1.))  => 0
        test_4; ray!((0., 2., -5.),  (0., 0., 1.))  => 0
        test_5; ray!((0., 1., -5.),  (0., 0., 1.))  => 0
        test_6; ray!((0., 1.5, 0.),  (0., 0., 1.))  => 2
    );
}

#[cfg(test)]
mod cylinder_cap_intersection_tests {
    use crate::intersection::Intersect;

    use crate::ray;
    use math::matrix::matrix_4x4::*;

    macro_rules! intersect_cap {
        ($($name:ident; $ray:expr => $t:expr)*) => {
            $(
                #[test]
                fn $name() {
                    // cylinder 1..2
                    let cylinder = $crate::cylinder!(matrix: Matrix4x4::translation(0.,1.5,0.).scale(1.,0.5,1.)).to_intersectable();
                    let intersections = cylinder.intersect(&$ray.normalize());
                    assert_eq!(intersections.iter().map(|a|a.t).collect::<Vec<f32>>(), $t);
                    assert!(cylinder.fast_hit(&$ray))
                }
            )*
        };
    }

    intersect_cap!(
        test_1; ray!((0., 3., 0.),  (0., -1., 0.)) => vec!(1., 2.)
        test_2; ray!((0., 3., -2.),  (0., -1., 2.)) => vec!(2.236068, 3.354102)
        test_3; ray!((0., 4., -2.),  (0., -1., 1.)) => vec!(2.828427, 4.242641)
        test_4; ray!((0., 0., -2.),  (0., 1., 2.)) => vec!(2.236068, 3.354102)
        test_5; ray!((0., -1., -2.),  (0., 1., 1.)) => vec!(2.828427, 4.242641)
    );
}

#[cfg(test)]
mod cylinder_cap_normal_tests {

    use math::matrix::matrix_4x4::*;
    use math::{assert_vector, point, vector};

    macro_rules! normal_cap {
        ($($name:ident; $point:expr => $expected:expr)*) => {
            $(
                #[test]
                fn $name() {
                    // cylinder 1..2
                    let cylinder = $crate::cylinder!(matrix: Matrix4x4::translation(0.,1.5,0.).scale(1.,0.5,1.)).to_intersectable();
                    assert_vector!(cylinder.normal_at($point.into()).to_vector(), $expected);
                }
            )*
        };
    }

    normal_cap!(
        test_1; point!(0, 1, 0)   => vector!(0, -1, 0)
        test_2; point!(0.5, 1, 0) => vector!(0, -1, 0)
        test_3; point!(0, 1, 0.5) => vector!(0, -1, 0)
        test_4; point!(0, 2, 0)   => vector!(0, 1, 0)
        test_5; point!(0.5, 2, 0) => vector!(0, 1, 0)
        test_6; point!(0, 2, 0.5) => vector!(0, 1, 0)

        a; point!(1, 0, 0) => vector!(1, 0, 0)
        b; point!(0, 5, -1) => vector!(0, 0, -1)
        c; point!(0, -2, 1) => vector!(0, 0, 1)
        d; point!(-1, 1, 0) => vector!(-1, 0, 0)
    );
}
