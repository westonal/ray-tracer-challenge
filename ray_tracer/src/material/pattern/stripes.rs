use crate::Transform::Transform;
use crate::material::pattern::Pattern;
use math::tuple::color::Color;
use math::tuple::point::Point;

impl Pattern {
    pub(crate) fn stripe_color_at(
        a: &Color,
        b: &Color,
        pattern_transform: &Transform,
        object_point: Point,
    ) -> Color {
        let point = pattern_transform.world_point_to_object_point(object_point);
        if point.x.floor() as i32 % 2 == 0 {
            a
        } else {
            b
        }
        .clone()
    }
}

#[cfg(test)]
mod stripe_pattern_tests {
    use super::*;
    use crate::material::pattern::Pattern::Stripe;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::tuple::color::{BLACK, WHITE};
    use math::{degrees, point};

    #[test]
    fn a_stripe_pattern_is_constant_in_y_and_z() {
        let pattern = Stripe(*WHITE, *BLACK, Transform::identity());
        assert_eq!(
            pattern.color_at(point!(0, 0, 0), &Transform::identity()),
            *WHITE
        );
        assert_eq!(
            pattern.color_at(point!(0, 1, 0), &Transform::identity()),
            *WHITE
        );
        assert_eq!(
            pattern.color_at(point!(0, 0, 1), &Transform::identity()),
            *WHITE
        );
        assert_eq!(
            pattern.color_at(point!(0, 1, 1), &Transform::identity()),
            *WHITE
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Stripe(*WHITE, *BLACK, Transform::identity());
        assert_eq!(
            pattern.color_at(point!(0, 0, 0), &Transform::identity()),
            *WHITE
        );
        assert_eq!(
            pattern.color_at(point!(1, 0, 0), &Transform::identity()),
            *BLACK
        );
        assert_eq!(
            pattern.color_at(point!(2, 0, 1), &Transform::identity()),
            *WHITE
        );
        assert_eq!(
            pattern.color_at(point!(3, 0, 1), &Transform::identity()),
            *BLACK
        );
        assert_eq!(
            pattern.color_at(point!(-0.9, 0, 0), &Transform::identity()),
            *BLACK
        );
        assert_eq!(
            pattern.color_at(point!(-1.1, 0, 0), &Transform::identity()),
            *WHITE
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_y_after_rotation() {
        let pattern = Stripe(
            *WHITE,
            *BLACK,
            Transform::new(Matrix4x4::rotation_z(degrees!(90))),
        );
        assert_eq!(
            pattern.color_at(point!(0, 0, 0), &Transform::identity()),
            *WHITE
        );
        assert_eq!(
            pattern.color_at(point!(0, 1, 0), &Transform::identity()),
            *BLACK
        );
        assert_eq!(
            pattern.color_at(point!(0, 2, 1), &Transform::identity()),
            *WHITE
        );
        assert_eq!(
            pattern.color_at(point!(0, 3, 1), &Transform::identity()),
            *BLACK
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_y_after_outer_rotation() {
        let pattern = Stripe(*WHITE, *BLACK, Transform::identity());
        let transform = Transform::new(Matrix4x4::rotation_z(degrees!(90)));
        assert_eq!(pattern.color_at(point!(0, 0, 0), &transform), *WHITE);
        assert_eq!(pattern.color_at(point!(0, 1, 0), &transform), *BLACK);
        assert_eq!(pattern.color_at(point!(0, 2, 1), &transform), *WHITE);
        assert_eq!(pattern.color_at(point!(0, 3, 1), &transform), *BLACK);
    }
}
