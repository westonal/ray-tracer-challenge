mod point;
mod vector;

use crate::tuple::point::Point;
use crate::tuple::vector::Vector;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
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
