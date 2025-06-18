use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub(crate) fn point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }
    pub(crate) fn vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
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

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
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
}
