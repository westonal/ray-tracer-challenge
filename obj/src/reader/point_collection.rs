use crate::{ObjPointIndex, ObjTriangle};
use math::tuple::point::Point;
use std::ops::{Deref, Index};

#[derive(Debug, PartialEq)]
pub struct PointCollection(Vec<Point>);

impl Deref for PointCollection {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index<ObjPointIndex> for PointCollection {
    type Output = Point;

    fn index(&self, index: ObjPointIndex) -> &Self::Output {
        self.0.get(index.0).unwrap()
    }
}

impl PointCollection {
    pub fn of(&self, triangle: &ObjTriangle) -> [Point; 3] {
        triangle.indicies.map(|f| self[f])
    }
}

impl Default for PointCollection {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl From<Vec<Point>> for PointCollection {
    fn from(value: Vec<Point>) -> Self {
        Self(value)
    }
}
