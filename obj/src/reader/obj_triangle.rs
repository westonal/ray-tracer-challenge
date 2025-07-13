use crate::ObjPointIndex;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct ObjTriangle {
    indicies: [ObjPointIndex; 3],
}

impl From<[ObjPointIndex; 3]> for ObjTriangle {
    fn from(value: [ObjPointIndex; 3]) -> Self {
        ObjTriangle { indicies: value }
    }
}

impl Deref for ObjTriangle {
    type Target = [ObjPointIndex; 3];

    fn deref(&self) -> &Self::Target {
        &self.indicies
    }
}
