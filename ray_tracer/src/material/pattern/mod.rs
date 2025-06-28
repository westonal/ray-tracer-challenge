mod stripes;

use crate::Transform::Transform;
use crate::material::pattern::Pattern::Solid;
use math::tuple::color::{Color, WHITE};
use math::tuple::point::Point;

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Solid(Color),
    Stripe(Color, Color, Transform),
}

impl Default for Pattern {
    fn default() -> Self {
        Solid(*WHITE)
    }
}

impl Pattern {
    pub(crate) fn color_at(&self, point: Point, transform: &Transform) -> Color {
        match self {
            Solid(c) => c.clone(),
            Pattern::Stripe(a, b, pattern_transform) => Self::stripe_color_at(
                a,
                b,
                pattern_transform,
                transform.world_point_to_object_point(point),
            ),
        }
    }
}
