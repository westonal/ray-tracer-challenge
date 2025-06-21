use crate::tuple::Tuple;
use crate::tuple::vector::Vector;
use std::fmt::Formatter;
use std::ops::{Add, Deref, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    tuple: Tuple,
}

impl Point {
    pub fn origin() -> Self {
        Self::point(0., 0., 0.)
    }

    pub fn point(x: f32, y: f32, z: f32) -> Point {
        Self {
            tuple: Tuple::new(x, y, z, 1.),
        }
    }
}

impl From<(f32, f32, f32)> for Point {
    fn from(tuple: (f32, f32, f32)) -> Point {
        Point::point(tuple.0, tuple.1, tuple.2)
    }
}

impl From<Point> for Tuple {
    fn from(value: Point) -> Self {
        value.tuple
    }
}

impl Deref for Point {
    type Target = Tuple;

    fn deref(&self) -> &Self::Target {
        &self.tuple
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Pt({}, {}, {}, {})",
            self.x, self.y, self.z, self.w
        ))
    }
}

#[cfg(test)]
mod point_display_tests {
    use super::*;

    #[test]
    fn tuple_display_point() {
        let point = Tuple::point(8., 9.5, -4.);

        assert_eq!("Pt(8, 9.5, -4, 1)", format!("{point}"));
    }
}

#[cfg(test)]
mod point_into_tests {
    use super::*;
    #[test]
    fn tuple_3_into_vector() {
        assert_eq!(Point::point(12., 3., -18.7), (12., 3., -18.7).into());
    }
}

#[cfg(test)]
mod point_origin {
    use super::*;
    #[test]
    fn origin() {
        assert_eq!(Point::point(0., 0., 0.), Point::origin());
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Tuple::point(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::vector(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Tuple::point(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod point_math_tests {
    use super::*;

    #[test]
    fn add_point_to_a_vector_yields_a_point() {
        let tuple1 = Point::point(1., 2., 3.);
        let tuple2 = Tuple::vector(4., 5., 6.);

        assert_eq!(tuple1 + tuple2, Point::point(5., 7., 9.));
    }

    #[test]
    fn sub_point_from_point_yields_a_vector() {
        let tuple1 = Point::point(1., 5., 4.);
        let tuple2 = Point::point(4., 1., 6.);

        assert_eq!(tuple1 - tuple2, Tuple::vector(-3., 4., -2.));
    }

    #[test]
    fn sub_vector_from_point_yields_a_point() {
        let tuple1 = Point::point(1., 5., 4.);
        let tuple2 = Tuple::vector(4., 1., 6.);

        assert_eq!(tuple1 - tuple2, Point::point(-3., 4., -2.));
    }
}
