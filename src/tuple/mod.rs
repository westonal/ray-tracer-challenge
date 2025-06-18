mod point;
mod vector;

use crate::tuple::point::Point;
use crate::tuple::vector::Vector;
use std::fmt::Formatter;
use std::ops::{Add, Neg, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub(crate) fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
}

impl Tuple {
    pub(crate) fn point(x: f32, y: f32, z: f32) -> Point {
        Point::point(x, y, z)
    }
    pub(crate) fn vector(x: f32, y: f32, z: f32) -> Vector {
        Vector::vector(x, y, z)
    }
    pub(crate) fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

#[cfg(test)]
mod tuple_creation_tests {
    use super::*;

    #[test]
    fn tuple_creation() {
        let tuple = Tuple::new(2.0, 3.0, 4.0, 1.0);
        assert_eq!(tuple.x, 2.0);
        assert_eq!(tuple.y, 3.0);
        assert_eq!(tuple.z, 4.0);
        assert_eq!(tuple.w, 1.0);
    }

    #[test]
    fn vector_creation() {
        let tuple: Tuple = *Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(tuple.x, 2.0);
        assert_eq!(tuple.y, 3.0);
        assert_eq!(tuple.z, 4.0);
        assert_eq!(tuple.w, 0.0);
    }

    #[test]
    fn point_creation() {
        let tuple = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(tuple.x, 2.0);
        assert_eq!(tuple.y, 3.0);
        assert_eq!(tuple.z, 4.0);
        assert_eq!(tuple.w, 1.0);
    }
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({}, {}, {}, {})",
            self.x, self.y, self.z, self.w
        ))
    }
}

#[cfg(test)]
mod tuple_display_tests {
    use super::*;
    #[test]
    fn tuple_display() {
        let vector = Tuple::new(12.0, 3.0, -18.7, 5.0);

        assert_eq!("(12, 3, -18.7, 5)", format!("{vector}"));
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
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

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Tuple::vector(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Tuple::vector(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tuple_math_tests {
    use super::*;

    #[test]
    fn add_point_to_a_vector_yields_a_point() {
        let tuple1 = Tuple::point(1.0, 2.0, 3.0);
        let tuple2 = Tuple::vector(4.0, 5.0, 6.0);

        assert_eq!(tuple1 + tuple2, Tuple::point(5.0, 7.0, 9.0));
    }

    #[test]
    fn sub_point_from_point_yields_a_vector() {
        let tuple1 = Tuple::point(1.0, 5.0, 4.0);
        let tuple2 = Tuple::point(4.0, 1.0, 6.0);

        assert_eq!(tuple1 - tuple2, Tuple::vector(-3.0, 4.0, -2.0));
    }

    #[test]
    fn sub_vector_from_point_yields_a_point() {
        let tuple1 = Tuple::point(1.0, 5.0, 4.0);
        let tuple2 = Tuple::vector(4.0, 1.0, 6.0);

        assert_eq!(tuple1 - tuple2, Tuple::point(-3.0, 4.0, -2.0));
    }

    #[test]
    fn sub_vector_from_vector_yields_a_vector() {
        let tuple1 = Tuple::vector(1.0, 5.0, 4.0);
        let tuple2 = Tuple::vector(4.0, 1.0, 6.0);

        assert_eq!(tuple1 - tuple2, Tuple::vector(-3.0, 4.0, -2.0));
    }

    #[test]
    fn tuple_vector_negation() {
        let tuple1 = Tuple::vector(0.0, 0.0, 0.0);
        let tuple2 = Tuple::vector(4.0, 1.0, 6.0);

        assert_eq!(tuple1 - tuple2, -tuple2);
    }

    #[test]
    fn tuple_vector_scalar_multiply() {
        let tuple1 = Tuple::vector(8.0, 2.0, -12.0);
        let tuple2 = Tuple::vector(4.0, 1.0, -6.0);

        assert_eq!(tuple1, tuple2 * 2.0);
    }

    #[test]
    fn tuple_vector_divide() {
        let tuple1 = Tuple::vector(12.0, 3.0, -18.0);
        let tuple2 = Tuple::vector(4.0, 1.0, -6.0);

        assert_eq!(tuple2, tuple1 / 3.0);
    }

    #[test]
    fn tuple_magnitude_vector() {
        let tuple1 = Tuple::vector(3.0, 4.0, 0.0);

        assert_eq!(5.0, tuple1.magnitude());
    }

    #[test]
    fn tuple_magnitude_vector_2() {
        let tuple1 = Tuple::vector(7.0, 4.0, 4.0);

        assert_eq!(9.0, tuple1.magnitude());
    }

    #[test]
    fn tuple_magnitude_pure_tuple() {
        let tuple1 = Tuple::new(7.0, 4.0, 0.0, 4.0);

        assert_eq!(9.0, tuple1.magnitude());
    }
}
