use crate::material::pattern::Pattern;
use crate::transform::Transform;
use math::tuple::color::Color;
use math::tuple::point::Point;

impl Pattern {
    pub(crate) fn checker_color_at(
        a: &Color,
        b: &Color,
        pattern_transform: &Transform,
        object_point: Point,
    ) -> Color {
        let point = pattern_transform.world_point_to_object_point(object_point);
        let x = Self::convert(point.x);
        let y = Self::convert(point.y);
        let z = Self::convert(point.z);
        if (x + y + z) % 2 == 0 { a } else { b }.clone()
    }

    /// This complex conversion is to avoid some rash effect when the values are
    /// Bouncing near to a boundary. E.g. commonly seen on a plane with y=0
    fn convert(f: f32) -> i32 {
        ((f * 100000.).round() / 100000.).floor() as i32
    }
}

#[cfg(test)]
mod checker_pattern_tests {
    use super::*;
    use crate::material::pattern::Pattern::Checker;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::point;
    use math::tuple::color::{BLUE, RED};

    #[test]
    fn a_checker_pattern_alternates_in_x() {
        let pattern = Checker(*RED, *BLUE, Transform::identity());
        assert_eq!(
            pattern.color_at(point!(0, 0, 0), &Transform::identity()),
            *RED
        );
        assert_eq!(
            pattern.color_at(point!(1, 0, 0), &Transform::identity()),
            *BLUE
        );
        assert_eq!(
            pattern.color_at(point!(2, 0, 0), &Transform::identity()),
            *RED
        );
        assert_eq!(
            pattern.color_at(point!(3, 0, 0), &Transform::identity()),
            *BLUE
        );
    }

    #[test]
    fn a_checker_pattern_alternates_in_y() {
        let pattern = Checker(*RED, *BLUE, Transform::identity());
        assert_eq!(
            pattern.color_at(point!(0, 0, 0), &Transform::identity()),
            *RED
        );
        assert_eq!(
            pattern.color_at(point!(0, 1, 0), &Transform::identity()),
            *BLUE
        );
        assert_eq!(
            pattern.color_at(point!(0, 2, 0), &Transform::identity()),
            *RED
        );
        assert_eq!(
            pattern.color_at(point!(0, 3, 0), &Transform::identity()),
            *BLUE
        );
    }

    #[test]
    fn a_checker_pattern_alternates_in_z() {
        let pattern = Checker(*RED, *BLUE, Transform::identity());
        assert_eq!(
            pattern.color_at(point!(0, 0, 0), &Transform::identity()),
            *RED
        );
        assert_eq!(
            pattern.color_at(point!(0, 0, 1), &Transform::identity()),
            *BLUE
        );
        assert_eq!(
            pattern.color_at(point!(0, 0, 2), &Transform::identity()),
            *RED
        );
        assert_eq!(
            pattern.color_at(point!(0, 0, 3), &Transform::identity()),
            *BLUE
        );
    }

    #[test]
    fn a_checker_pattern_applies_the_supplied_object_transform() {
        let pattern = Checker(*RED, *BLUE, Transform::identity());
        let transform = &Transform::new(Matrix4x4::scale(2., 1., 1.));
        assert_eq!(pattern.color_at(point!(0, 0, 0), transform), *RED);
        assert_eq!(pattern.color_at(point!(1, 0, 0), transform), *RED);
        assert_eq!(pattern.color_at(point!(2, 0, 0), transform), *BLUE);
        assert_eq!(pattern.color_at(point!(3, 0, 0), transform), *BLUE);
    }

    #[test]
    fn a_checker_pattern_applies_the_pattern_transform() {
        let pattern = Checker(*RED, *BLUE, Transform::new(Matrix4x4::scale(2., 1., 1.)));
        assert_eq!(
            pattern.color_at(point!(0, 0, 0), &Transform::identity()),
            *RED
        );
        assert_eq!(
            pattern.color_at(point!(1, 0, 0), &Transform::identity()),
            *RED
        );
        assert_eq!(
            pattern.color_at(point!(2.1, 0, 0), &Transform::identity()),
            *BLUE
        );
        assert_eq!(
            pattern.color_at(point!(3.1, 0, 0), &Transform::identity()),
            *BLUE
        );
    }
}
