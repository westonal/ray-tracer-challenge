mod color;
mod point;
mod vector;

use crate::tuple::point::Point;
use crate::tuple::vector::Vector;
use std::fmt::Formatter;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub fn hadamard_product(&self, other: Tuple) -> Tuple {
        Self::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
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
        let tuple = Tuple::vector(2.0, 3.0, 4.0);
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
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

#[cfg(test)]
mod tuple_math_tests {

    use super::*;

    #[test]
    fn tuple_add() {
        let a = Tuple::new(0.9, 0.5, 0.75, 1.0);
        let b = Tuple::new(1.7, 0.3, 0.35, 1.2);

        assert_eq!(Tuple::new(2.6, 0.8, 1.1, 2.2), a + b)
    }

    #[test]
    fn tuple_sub() {
        let a = Tuple::new(1.8, 0.5, 0.75, 1.0);
        let b = Tuple::new(0.9, 0.1, 0.35, 0.4);

        assert_eq!(Tuple::new(0.9, 0.4, 0.4, 0.6), a - b)
    }

    #[test]
    fn tuple_multiply_scalar() {
        let a = Tuple::new(1.0, 0.5, 0.75, 0.8);

        assert_eq!(Tuple::new(3.0, 1.5, 2.25, 2.4), a * 3.0)
    }

    #[test]
    fn tuple_hadamard_product() {
        let a = Tuple::new(1.8, 0.5, 0.75, 1.2);
        let b = Tuple::new(2.0, 0.1, 0.35, 0.5);

        assert_eq!(Tuple::new(3.6, 0.05, 0.2625, 0.6), a.hadamard_product(b))
    }
}
