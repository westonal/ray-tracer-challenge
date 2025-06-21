use math::tuple::point::Point;
use math::tuple::vector::Vector;

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

#[cfg(test)]
mod ray_construction_tests {
    use super::*;

    #[test]
    fn create() {
        let ray = Ray::new((1., 2., 3.).into(), (4., 5., 6.).into());
        assert_eq!(ray.origin, Point::point(1., 2., 3.));
        assert_eq!(ray.direction, Vector::vector(4., 5., 6.));
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
