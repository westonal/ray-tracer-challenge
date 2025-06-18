use crate::tuple::vector::Vector;
use crate::tuple::Tuple;
use std::fmt::Formatter;
use std::ops::{Add, Deref, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    tuple: Tuple,
}

impl Point {
    pub(crate) fn point(x: f32, y: f32, z: f32) -> Point {
        Self {
            tuple: Tuple::new(x, y, z, 1.0),
        }
    }
}

impl From<(f32, f32, f32)> for Point {
    fn from(tuple: (f32, f32, f32)) -> Point {
        Point::point(tuple.0, tuple.1, tuple.2)
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
        let point = Tuple::point(8.0, 9.5, -4.0);

        assert_eq!("Pt(8, 9.5, -4, 1)", format!("{point}"));
    }
}

#[cfg(test)]
mod point_into_tests {
    use super::*;
    #[test]
    fn tuple_3_into_vector() {
        assert_eq!(Point::point(12.0, 3.0, -18.7), (12.0, 3.0, -18.7).into());
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
        let tuple1 = Point::point(1.0, 2.0, 3.0);
        let tuple2 = Tuple::vector(4.0, 5.0, 6.0);

        assert_eq!(tuple1 + tuple2, Point::point(5.0, 7.0, 9.0));
    }

    #[test]
    fn sub_point_from_point_yields_a_vector() {
        let tuple1 = Point::point(1.0, 5.0, 4.0);
        let tuple2 = Point::point(4.0, 1.0, 6.0);

        assert_eq!(tuple1 - tuple2, Tuple::vector(-3.0, 4.0, -2.0));
    }

    #[test]
    fn sub_vector_from_point_yields_a_point() {
        let tuple1 = Point::point(1.0, 5.0, 4.0);
        let tuple2 = Tuple::vector(4.0, 1.0, 6.0);

        assert_eq!(tuple1 - tuple2, Point::point(-3.0, 4.0, -2.0));
    }
}
