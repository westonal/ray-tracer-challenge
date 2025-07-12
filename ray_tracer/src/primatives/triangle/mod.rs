use math::tuple::point::Point;
use math::tuple::vector::Vector;
use math::tuple::vector::normal::Normal;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    e1: Vector,
    e2: Vector,
    pub(crate) normal: Normal,
}

impl Triangle {
    pub(crate) fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(e1).normalize();
        Self {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
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
        assert_vector!(vector!(0, 0, -1), triangle.normal.to_vector());
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
            shape.normal_at(point!(0, 0.5, 0)).to_vector()
        );
    }
}
