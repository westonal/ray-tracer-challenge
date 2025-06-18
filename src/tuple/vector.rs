use crate::tuple::Tuple;
use std::fmt::Formatter;
use std::ops::{Deref, Div, Mul};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    tuple: Tuple,
}

impl Vector {
    pub(crate) fn new(tuple: Tuple) -> Vector {
        Self { tuple }
    }

    pub(crate) fn vector(x: f32, y: f32, z: f32) -> Vector {
        Self {
            tuple: Tuple::new(x, y, z, 0.0),
        }
    }
}

impl From<(f32, f32, f32)> for Vector {
    fn from(tuple: (f32, f32, f32)) -> Vector {
        Vector::vector(tuple.0, tuple.1, tuple.2)
    }
}

impl Deref for Vector {
    type Target = Tuple;

    fn deref(&self) -> &Self::Target {
        &self.tuple
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("V{}", self.tuple))
    }
}

#[cfg(test)]
mod vector_display_tests {
    use super::*;
    #[test]
    fn tuple_display_vector() {
        let vector = Tuple::vector(12.0, 3.0, -18.7);

        assert_eq!("V(12, 3, -18.7, 0)", format!("{vector}"));
    }
}

#[cfg(test)]
mod vector_into_tests {
    use super::*;
    #[test]
    fn tuple_3_into_vector() {
        assert_eq!(Vector::vector(12.0, 3.0, -18.7), (12.0, 3.0, -18.7).into());
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(Tuple::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs,
        ))
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            tuple: self.tuple / rhs,
        }
    }
}
