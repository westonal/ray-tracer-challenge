use crate::material::pattern::Pattern::Solid;
use math::tuple::color::{Color, WHITE};
use math::tuple::point::Point;

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Solid(Color),
    Stripe(Color, Color),
}

impl Default for Pattern {
    fn default() -> Self {
        Solid(*WHITE)
    }
}

impl Pattern {
    pub(crate) fn color_at(&self, point: &Point) -> Color {
        match self {
            Solid(c) => c,
            Pattern::Stripe(a, b) => {
                if point.x as i32 % 2 == 0 {
                    a
                } else {
                    b
                }
            }
        }
        .clone()
    }
}

#[cfg(test)]
mod stripe_pattern_tests {
    use super::*;
    use crate::material::pattern::Pattern::Stripe;
    use math::point;
    use math::tuple::color::BLACK;

    #[test]
    fn a_stripe_pattern_is_constant_in_y_and_z() {
        let pattern = Stripe(*WHITE, *BLACK);
        assert_eq!(pattern.color_at(&point!(0, 0, 0)), *WHITE);
        assert_eq!(pattern.color_at(&point!(0, 1, 0)), *WHITE);
        assert_eq!(pattern.color_at(&point!(0, 0, 1)), *WHITE);
        assert_eq!(pattern.color_at(&point!(0, 1, 1)), *WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Stripe(*WHITE, *BLACK);
        assert_eq!(pattern.color_at(&point!(0, 0, 0)), *WHITE);
        assert_eq!(pattern.color_at(&point!(1, 0, 0)), *BLACK);
        assert_eq!(pattern.color_at(&point!(2, 0, 1)), *WHITE);
        assert_eq!(pattern.color_at(&point!(3, 0, 1)), *BLACK);
    }
}
