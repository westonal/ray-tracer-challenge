use math::tuple::point::Point;
use math::tuple::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub(crate) vertices: [Point; 3],
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
    pub fn new(vertices: [Point; 3]) -> Self {
        let p1 = vertices[0];
        let p2 = vertices[1];
        let p3 = vertices[2];
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(e1).normalize();
        Self {
            vertices,
            e1,
            e2,
            normal: normal.to_vector().into(),
        }
    }

    pub fn new_smooth(vertices: [Point; 3], normals: [Vector; 3]) -> Self {
        let p1 = vertices[0];
        let p2 = vertices[1];
        let p3 = vertices[2];
        Self {
            vertices,
            e1: p2 - p1,
            e2: p3 - p1,
            normal: TriangleNormal::PerVertex(normals),
        }
    }
}

impl From<[Point; 3]> for Triangle {
    fn from(value: [Point; 3]) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod create_triangle_tests {
    use super::*;
    use math::{assert_vector, point, vector};

    #[test]
    fn create() {
        let triangle = Triangle::new([point!(0, 1, 0), point!(-1, 0, 0), point!(1, 0, 0)]);
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
            [point!(0, 1, 0), point!(-1, 0, 0), point!(1, 0, 0)],
            [n1, n2, n3],
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
        let shape = Shape::new_triangle(Triangle::new([
            point!(0, 1, 0),
            point!(-1, 0, 0),
            point!(1, 0, 0),
        ]))
        .to_intersectable();
        assert_vector!(
            vector!(0, 0, -1),
            shape.normal_at(point!(0, 0.5, 0).into()).to_vector()
        );
    }
}
