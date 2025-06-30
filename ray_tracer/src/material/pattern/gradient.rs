use crate::material::pattern::Pattern;
use crate::transform::Transform;
use math::tuple::color::Color;
use math::tuple::point::Point;

#[derive(Debug, PartialEq)]
pub struct GradientStops(Vec<(f32, Color)>);

impl Pattern {
    pub(crate) fn gradient_color_at(
        stops: &GradientStops,
        pattern_transform: &Transform,
        point: Point,
    ) -> Color {
        stops.color_at(pattern_transform.world_point_to_object_point(point).x)
    }
}

impl GradientStops {
    pub(crate) fn color_at(&self, input: f32) -> Color {
        let mut first = self.0.get(0).unwrap();
        let mut last = self.0.get(0).unwrap();
        for e in &self.0 {
            let (f, _) = e;
            first = last;
            last = e;
            if f >= &input {
                break;
            }
        }
        let d = last.0 - first.0;
        if d <= 0. {
            return first.1;
        }
        let ratio = (input - first.0) / d;
        first.1.cross_blend(last.1.clone(), ratio)
    }
}

impl GradientStops {
    pub fn new(vec: Vec<(f32, Color)>) -> Self {
        Self(vec)
    }
}

#[macro_export]
macro_rules! gradient_stops {
    ($($x:expr => $y:expr),+ $(,)?) => {
        $crate::material::pattern::gradient::GradientStops::new(vec!($(($x as f32, $y)),+))
    };
}

#[cfg(test)]
mod gradient_stop_tests {

    use math::color;

    #[test]
    fn single_stop() {
        let gradient = gradient_stops!(0.1 => color!(1,0,0));
        assert_eq!(color!(1, 0, 0), gradient.color_at(0.));
        assert_eq!(color!(1, 0, 0), gradient.color_at(0.1));
        assert_eq!(color!(1, 0, 0), gradient.color_at(1.));
        assert_eq!(color!(1, 0, 0), gradient.color_at(0.5));
    }

    #[test]
    fn two_stops() {
        let gradient = gradient_stops!(0.1 => color!(1,0,0),0.9 => color!(1,1,0),);
        assert_eq!(color!(1, 0, 0), gradient.color_at(0.));
        assert_eq!(color!(1, 0, 0), gradient.color_at(0.1));
        assert_eq!(color!(1, 1, 0), gradient.color_at(0.9));
        assert_eq!(color!(1, 1, 0), gradient.color_at(1.));
        assert_eq!(color!(1, 0.50000006, 0), gradient.color_at(0.5));
    }

    #[test]
    fn three_stops() {
        let gradient = gradient_stops!(
            0.1 => color!(1,0,0),
            0.5 => color!(1,0,1),
            0.9 => color!(1,1,0),
        );
        assert_eq!(color!(1, 0, 0), gradient.color_at(0.));
        assert_eq!(color!(1, 0, 0), gradient.color_at(0.1));
        assert_eq!(color!(1, 0, 1), gradient.color_at(0.5));
        assert_eq!(color!(1, 1, 0), gradient.color_at(0.9));
        assert_eq!(color!(1, 1, 0), gradient.color_at(1.));
        assert_eq!(color!(1, 0, 0.50000006), gradient.color_at(0.3));
        assert_eq!(color!(1, 0.62500006, 0.37499994), gradient.color_at(0.75));
    }
}

#[cfg(test)]
mod gradient_pattern_tests {

    use crate::material::pattern::Pattern::Gradient;
    use crate::transform::Transform;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::tuple::color::{BLACK, BLUE, GREEN, WHITE};
    use math::{color, degrees, point};

    #[test]
    fn a_gradient_pattern_is_constant_in_y_and_z() {
        let pattern = Gradient(
            gradient_stops!(0. => *WHITE, 1.=>*BLACK),
            Transform::identity(),
        );
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
    fn a_gradient_pattern_applies_over_a_single_x_unit() {
        let pattern = Gradient(
            gradient_stops!(0. => *WHITE, 1.=>*BLACK),
            Transform::identity(),
        );
        assert_eq!(
            pattern.color_at(point!(-1, 0, 0), &Transform::identity()),
            *WHITE
        );
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
            *BLACK
        );
        assert_eq!(
            pattern.color_at(point!(0.5, 0, 1), &Transform::identity()),
            color!(0.5, 0.5, 0.5)
        );
    }

    #[test]
    fn a_gradient_pattern_applies_over_a_single_y_unit_after_rotation() {
        let pattern = Gradient(
            gradient_stops!(0. => *GREEN, 1.=>*BLUE),
            Transform::new(Matrix4x4::rotation_z(degrees!(90))),
        );
        assert_eq!(
            pattern.color_at(point!(0, -1, 0), &Transform::identity()),
            *GREEN
        );
        assert_eq!(
            pattern.color_at(point!(0, 0, 0), &Transform::identity()),
            *GREEN
        );
        assert_eq!(
            pattern.color_at(point!(0, 1, 0), &Transform::identity()),
            *BLUE
        );
        assert_eq!(
            pattern.color_at(point!(0, 2, 1), &Transform::identity()),
            *BLUE
        );
        assert_eq!(
            pattern.color_at(point!(0, 0.75, 1), &Transform::identity()),
            color!(0., 0.25, 0.75)
        );
    }

    #[test]
    fn a_gradient_pattern_applies_in_y_after_outer_rotation() {
        let pattern = Gradient(
            gradient_stops!(0. => *GREEN, 1.=>*BLUE),
            Transform::identity(),
        );
        let transform = Transform::new(Matrix4x4::rotation_z(degrees!(90)));
        assert_eq!(pattern.color_at(point!(0, -1, 0), &transform), *GREEN);
        assert_eq!(pattern.color_at(point!(0, 0, 0), &transform), *GREEN);
        assert_eq!(pattern.color_at(point!(0, 1, 0), &transform), *BLUE);
        assert_eq!(pattern.color_at(point!(0, 2, 1), &transform), *BLUE);
        assert_eq!(
            pattern.color_at(point!(0, 0.75, 1), &transform),
            color!(0., 0.25, 0.75)
        );
    }
}
