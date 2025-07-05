mod assertions;
pub mod normal;
mod reflection;

use crate::tuple::Tuple;
use crate::tuple::vector::normal::Normal;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Deref, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    tuple: Tuple,
}

impl Vector {
    pub fn new(tuple: Tuple) -> Vector {
        Self { tuple }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Vector {
        Self {
            tuple: Tuple::new(x, y, z, 0.),
        }
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Vector) -> Self {
        let a = self;
        let b = other;
        Self::vector(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn magnitude(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Normal {
        Normal::new(self.clone())
    }
}

impl From<(f32, f32, f32)> for Vector {
    fn from(tuple: (f32, f32, f32)) -> Vector {
        Vector::vector(tuple.0, tuple.1, tuple.2)
    }
}

impl From<Vector> for Tuple {
    fn from(value: Vector) -> Self {
        value.tuple
    }
}

impl TryFrom<Tuple> for Vector {
    type Error = String;

    fn try_from(value: Tuple) -> Result<Self, Self::Error> {
        if value.w != 0. {
            Err(format!("Invalid vector {:?} w component is not 0.", value))
        } else {
            Ok(Vector::vector(value.x, value.y, value.z))
        }
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

#[macro_export]
macro_rules! vector {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::tuple::vector::Vector::vector($x as f32, $y as f32, $z as f32)
    };
}

#[cfg(test)]
mod vector_creation_tests {
    use super::*;
    #[test]
    fn macro_test() {
        assert_eq!(Vector::vector(12., 3., -18.7), vector!(12., 3., -18.7));
    }
}

#[cfg(test)]
mod vector_display_tests {
    use super::*;
    #[test]
    fn tuple_display_vector() {
        let vector = Tuple::vector(12., 3., -18.7);

        assert_eq!("V(12, 3, -18.7, 0)", format!("{vector}"));
    }
}

#[cfg(test)]
mod vector_into_tests {

    use super::*;
    #[test]
    fn tuple_3_into_vector() {
        assert_eq!(Vector::vector(12., 3., -18.7), (12., 3., -18.7).into());
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            tuple: self.tuple * rhs,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(Tuple::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
            self.w / rhs,
        ))
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            tuple: self.tuple + rhs.tuple,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.tuple += rhs.tuple;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            tuple: self.tuple - rhs.tuple,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Tuple::vector(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod vector_math_tests {
    use super::*;
    use crate::assert_vector;

    #[test]
    fn add_vector_from_vector_yields_a_vector() {
        let a = Vector::vector(1., 5., 4.);
        let b = Vector::vector(4., 1., 6.);

        assert_vector!(Vector::vector(5., 6., 10.), a + b);
    }

    #[test]
    fn add_assign_vector() {
        let mut a = Vector::vector(1., 5., 4.);
        let b = Vector::vector(4., 1., 6.);
        a += b;

        assert_vector!(Vector::vector(5., 6., 10.), a);
    }

    #[test]
    fn sub_vector_from_vector_yields_a_vector() {
        let a = Vector::vector(1., 5., 4.);
        let b = Vector::vector(4., 1., 6.);

        assert_vector!(Vector::vector(-3., 4., -2.), a - b);
    }

    #[test]
    fn tuple_vector_negation() {
        let zero = Vector::vector(0., 0., 0.);
        let a = Vector::vector(4., 1., 6.);

        assert_eq!(zero - a, -a);
    }

    #[test]
    fn vector_scalar_multiply() {
        let a = Vector::vector(4., 1., -6.);

        assert_vector!(Vector::vector(8., 2., -12.), a * 2.);
    }

    #[test]
    fn vector_divide() {
        let a = Vector::vector(12., 3., -18.);

        assert_vector!(Vector::vector(4., 1., -6.), a / 3.);
    }

    #[test]
    fn vector_magnitude() {
        let a = Vector::vector(3., 4., 0.);

        assert_eq!(5., a.magnitude());
    }

    #[test]
    fn vector_magnitude_2() {
        let a = Vector::vector(7., 4., 4.);

        assert_eq!(9., a.magnitude());
    }

    #[test]
    fn tuple_magnitude_vector_with_w() {
        let a = Vector::new(Tuple::new(7., 4., 0., 4.));

        assert_eq!(9., a.magnitude());
    }

    #[test]
    fn vector_normalize() {
        let a = Vector::vector(4., 0., 0.);

        let normal: Normal = a.normalize();
        assert_eq!(&Vector::vector(1., 0., 0.), normal.deref());
    }

    #[test]
    fn vector_normalize_2() {
        let a = Vector::vector(1., 2., 3.);

        let normal: Normal = a.normalize();
        assert_eq!(
            &Vector::vector(0.26726124, 0.5345225, 0.8017837),
            normal.deref()
        );
    }

    #[test]
    fn vector_dot_product() {
        let a = Vector::vector(1., 2., 3.);
        let b = Vector::vector(2., 3., 4.);

        assert_eq!(20., a.dot(&b));
    }

    #[test]
    fn vector_dot_product_includes_w() {
        let a = Vector::new(Tuple::new(1., 2., 3., 4.));
        let b = Vector::new(Tuple::new(2., 3., 4., 5.));

        assert_eq!(40., a.dot(&b));
    }

    #[test]
    fn vector_cross_product() {
        let a = Vector::vector(1., 2., 3.);
        let b = Vector::vector(2., 3., 4.);

        assert_vector!(Vector::vector(-1., 2., -1.), a.cross(b));
        assert_vector!(Vector::vector(1., -2., 1.), b.cross(a));
    }
}

#[cfg(test)]
mod tuple_casting_tests {
    use super::*;
    use crate::assert_vector;
    #[test]
    fn valid_vector() {
        let vector: Vector = Tuple::new(1., 2., 3., 0.).try_into().unwrap();
        assert_vector!(Vector::vector(1., 2., 3.), vector);
    }

    #[test]
    fn invalid_point() {
        let result: Result<Vector, _> = Tuple::new(1., 2., 3., 1.).try_into();
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Invalid vector"));
    }

    #[test]
    fn invalid_point_forced_to_vector() {
        let vector: Vector = Tuple::new(1., 2., 3., 1.).force_vector();
        assert_vector!(vector!(1., 2., 3.), vector);
    }
}
