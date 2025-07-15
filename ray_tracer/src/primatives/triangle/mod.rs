use math::tuple::point::Point;
use math::tuple::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub(crate) p1: Point,
    p2: Point,
    p3: Point,
    pub(crate) e1: Vector,
    pub(crate) e2: Vector,
    pub(crate) normal: TriangleNormal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TriangleNormal {
    /// A single normal vector uniform over the whole surface
    Uniform(Vector),

    /// 3 normals defined at the points
    PerVertex([Vector; 3]),
}

impl From<Vector> for TriangleNormal {
    fn from(value: Vector) -> Self {
        Self::Uniform(value)
    }
}

impl Triangle {
    /// Triangle with a uniform normal
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(e1).normalize();
        Self {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal: normal.to_vector().into(),
        }
    }

    pub fn new_smooth(p1: Point, p2: Point, p3: Point, n1: Vector, n2: Vector, n3: Vector) -> Self {
        Self {
            p1,
            p2,
            p3,
            e1: p2 - p1,
            e2: p3 - p1,
            normal: TriangleNormal::PerVertex([n1, n2, n3]),
        }
    }
}

#[cfg(test)]
mod create_triangle_tests {
    use super::*;
    use math::{assert_vector, point, vector};

    #[test]
    fn create() {
        let triangle = Triangle::new(point!(0, 1, 0), point!(-1, 0, 0), point!(1, 0, 0));
        assert_vector!(vector!(-1, -1, 0), triangle.e1);
        assert_vector!(vector!(1, -1, 0), triangle.e2);
        assert_eq!(TriangleNormal::Uniform(vector!(0, 0, -1)), triangle.normal);
    }

    #[test]
    fn create_smooth() {
        let n1 = vector!(0, 1, 0);
        let n2 = vector!(-1, 0, 0);
        let n3 = vector!(1, 0, 0);
        let triangle = Triangle::new_smooth(
            point!(0, 1, 0),
            point!(-1, 0, 0),
            point!(1, 0, 0),
            n1,
            n2,
            n3,
        );
        assert_vector!(vector!(-1, -1, 0), triangle.e1);
        assert_vector!(vector!(1, -1, 0), triangle.e2);
        assert_eq!(TriangleNormal::PerVertex([n1, n2, n3]), triangle.normal);
    }
}

#[cfg(test)]
mod normal_tests {
    use super::*;
    use crate::primatives::Shape;
    use math::{assert_vector, point, vector};

    #[test]
    fn assert_fixed_normal() {
        let shape = Shape::new_triangle(Triangle::new(
            point!(0, 1, 0),
            point!(-1, 0, 0),
            point!(1, 0, 0),
        ))
        .to_intersectable();
        assert_vector!(
            vector!(0, 0, -1),
            shape.normal_at(point!(0, 0.5, 0).into()).to_vector()
        );
    }
}
