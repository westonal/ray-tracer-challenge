pub mod gradient;
mod stripes;

use crate::Transform::Transform;
use crate::material::pattern::gradient::GradientStops;
use math::tuple::color::{Color, WHITE};
use math::tuple::point::Point;

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Solid(Color),
    Stripe(Color, Color, Transform),
    Gradient(GradientStops, Transform),
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern::Solid(*WHITE)
    }
}

impl Pattern {
    pub(crate) fn color_at(&self, point: Point, transform: &Transform) -> Color {
        match self {
            Pattern::Solid(c) => c.clone(),
            Pattern::Stripe(a, b, pattern_transform) => Self::stripe_color_at(
                a,
                b,
                pattern_transform,
                transform.world_point_to_object_point(point),
            ),
            Pattern::Gradient(stops, pattern_transform) => Self::gradient_color_at(
                stops,
                pattern_transform,
                transform.world_point_to_object_point(point),
            ),
        }
    }
}
