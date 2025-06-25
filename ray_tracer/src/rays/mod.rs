mod transform;

use math::tuple::point::Point;
use math::tuple::vector::Vector;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}

#[macro_export]
macro_rules! ray {
    ($point: expr, $direction: expr) => {
        $crate::rays::Ray::new($point.into(), $direction.into())
    };
}

#[cfg(test)]
mod ray_construction_tests {
    use super::*;
    use math::{point, vector};

    #[test]
    fn create() {
        let ray = Ray::new((1., 2., 3.).into(), (4., 5., 6.).into());
        assert_eq!(ray.origin, point!(1., 2., 3.));
        assert_eq!(ray.direction, vector!(4., 5., 6.));
    }

    #[test]
    fn create_by_macro() {
        let ray = Ray::new((1., 2., 3.).into(), (4., 5., 6.).into());
        let ray2 = ray!((1., 2., 3.), (4., 5., 6.));
        assert_eq!(ray, ray2);
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "R[{} -> {}]", self.origin, self.direction)
    }
}

impl Debug for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod ray_display_tests {
    use crate::rays::Ray;

    #[test]
    fn display_ray() {
        assert_eq!(
            "R[Pt(1, 2, 3, 1) -> V(4, 5, 6, 0)]",
            format!("{}", ray!((1., 2., 3.), (4., 5., 6.)))
        );
    }
}
#[cfg(test)]
mod ray_tracing_at_time_tests {
    use super::*;

    #[test]
    fn time_0() {
        let ray = Ray::new((2., 3., 4.).into(), (1., 2., 3.).into());
        assert_eq!(ray.position(0.), Point::point(2., 3., 4.));
    }

    #[test]
    fn time_1() {
        let ray = Ray::new((2., 3., 4.).into(), (1., 2., 3.).into());
        assert_eq!(ray.position(1.), Point::point(3., 5., 7.));
    }

    #[test]
    fn time_minus_1() {
        let ray = Ray::new((2., 3., 4.).into(), (1., 2., 3.).into());
        assert_eq!(ray.position(-1.), Point::point(1., 1., 1.));
    }
    #[test]
    fn time_2_5() {
        let ray = Ray::new((2., 3., 4.).into(), (1., 2., 3.).into());
        assert_eq!(ray.position(2.5), Point::point(4.5, 8., 11.5));
    }
}
