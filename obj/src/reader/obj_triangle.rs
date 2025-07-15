use crate::{point_to_normal_index, ObjNormalIndex, ObjPointIndex};
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct ObjTriangle {
    indicies: [ObjPointIndex; 3],
    pub normal_indicies: Option<[ObjNormalIndex; 3]>,
}

impl From<[ObjPointIndex; 3]> for ObjTriangle {
    fn from(value: [ObjPointIndex; 3]) -> Self {
        ObjTriangle {
            indicies: value,
            normal_indicies: None,
        }
    }
}

impl From<([ObjPointIndex; 3], [ObjNormalIndex; 3])> for ObjTriangle {
    fn from(value: ([ObjPointIndex; 3], [ObjNormalIndex; 3])) -> Self {
        ObjTriangle {
            indicies: value.0,
            normal_indicies: Some(value.1),
        }
    }
}

impl Deref for ObjTriangle {
    type Target = [ObjPointIndex; 3];

    fn deref(&self) -> &Self::Target {
        &self.indicies
    }
}
