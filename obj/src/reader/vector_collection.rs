use crate::{ObjNormalIndex, ObjTriangle, PointCollection};
use math::tuple::point::Point;
use math::tuple::vector::Vector;
use std::ops::{Deref, Index};

#[derive(Debug, PartialEq)]
pub struct VectorCollection(Vec<Vector>);

impl Deref for VectorCollection {
    type Target = Vec<Vector>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index<ObjNormalIndex> for VectorCollection {
    type Output = Vector;

    fn index(&self, index: ObjNormalIndex) -> &Self::Output {
        self.0.get(index.0).unwrap()
    }
}

impl VectorCollection {
    pub fn of(&self, triangle: &ObjTriangle) -> Option<[Vector; 3]> {
        triangle.normal_indicies.map(|i| i.map(|f| self[f]))
    }
}

impl Default for VectorCollection {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl From<Vec<Vector>> for VectorCollection {
    fn from(value: Vec<Vector>) -> Self {
        Self(value)
    }
}
