use crate::matrix::matrix_4x4::Matrix4x4;
use crate::tuple::Tuple;
use crate::tuple::point::Point;
use crate::tuple::vector::Vector;
use std::ops::Mul;

impl Matrix4x4 {
    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4x4 {
        let mut m = Self::identity();
        m[0][3] = x;
        m[1][3] = y;
        m[2][3] = z;
        m
    }

    pub fn pre_translation(&self, x: f32, y: f32, z: f32) -> Matrix4x4 {
        self.clone() * Matrix4x4::translation(x, y, z)
    }
}

impl Mul<Point> for Matrix4x4 {
    type Output = Tuple;

    fn mul(self, rhs: Point) -> Self::Output {
        let t: Tuple = rhs.into();
        self * t
    }
}

impl Mul<Vector> for Matrix4x4 {
    type Output = Tuple;

    fn mul(self, rhs: Vector) -> Self::Output {
        let t: Tuple = rhs.into();
        self * t
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::matrix_4x4::Matrix4x4;

    use crate::{point, vector};

    #[test]
    fn translate_point() {
        let m = Matrix4x4::translation(5., -3., 2.);
        let p = point!(-3., 4., 5.);
        assert_eq!(m * p, point!(2., 1., 7.).into());
    }

    #[test]
    fn translate_point_fluent() {
        let m = Matrix4x4::identity().pre_translation(5., -3., 2.);
        let p = point!(-3., 4., 5.);
        assert_eq!(m * p, point!(2., 1., 7.).into());
    }

    #[test]
    fn translate_vector_does_not_affect_vector() {
        let m = Matrix4x4::translation(5., -3., 2.);
        let p = vector!(-3., 4., 5.);
        assert_eq!(m * p, p.into());
    }
}
