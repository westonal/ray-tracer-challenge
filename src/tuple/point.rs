use crate::tuple::Tuple;
use std::fmt::Formatter;
use std::ops::Deref;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    tuple: Tuple,
}

impl Point {
    pub(crate) fn new(tuple: Tuple) -> Point {
        Self { tuple }
    }

    pub(crate) fn point(x: f32, y: f32, z: f32) -> Point {
        Self {
            tuple: Tuple::new(x, y, z, 1.0),
        }
    }
}

impl From<(f32, f32, f32)> for Point {
    fn from(tuple: (f32, f32, f32)) -> Point {
        Point::point(tuple.0, tuple.1, tuple.2)
    }
}

impl Deref for Point {
    type Target = Tuple;

    fn deref(&self) -> &Self::Target {
        &self.tuple
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Pt({}, {}, {}, {})",
            self.x, self.y, self.z, self.w
        ))
    }
}

#[cfg(test)]
mod point_display_tests {
    use super::*;

    #[test]
    fn tuple_display_point() {
        let point = Tuple::point(8.0, 9.5, -4.0);

        assert_eq!("Pt(8, 9.5, -4, 1)", format!("{point}"));
    }
}

#[cfg(test)]
mod point_into_tests {
    use super::*;
    #[test]
    fn tuple_3_into_vector() {
        assert_eq!(Point::point(12.0, 3.0, -18.7), (12.0, 3.0, -18.7).into());
    }
}
