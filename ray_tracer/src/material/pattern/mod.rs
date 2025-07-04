pub mod checker;
pub mod gradient;
mod stripes;

use crate::material::pattern::gradient::GradientStops;
use crate::transform::Transform;
use math::color;
use math::tuple::color::{Color, WHITE};
use math::tuple::point::Point;

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Solid(Color),
    Stripe(Color, Color, Transform),

    /// The gradient runs from 0..1 in the x-axis
    Gradient(GradientStops, Transform),

    Checker(Color, Color, Transform),
    Test,
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
            Pattern::Checker(a, b, pattern_transform) => Self::checker_color_at(
                a,
                b,
                pattern_transform,
                transform.world_point_to_object_point(point),
            ),
            Pattern::Test => {
                let t = transform.world_point_to_object_point(point);
                color!(t.x, t.y, t.z)
            }
        }
    }
}
