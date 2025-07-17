use crate::{ObjNormalIndex, ObjPointIndex};
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct ObjTriangle {
    pub indicies: [ObjPointIndex; 3],
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

impl From<([ObjPointIndex; 3], [Option<ObjNormalIndex>; 3])> for ObjTriangle {
    fn from(value: ([ObjPointIndex; 3], [Option<ObjNormalIndex>; 3])) -> Self {
        let normals: Option<Vec<ObjNormalIndex>> = value.1.into_iter().collect();
        let normals = normals.map(|f| [f[0], f[1], f[2]]);
        ObjTriangle {
            indicies: value.0,
            normal_indicies: normals,
        }
    }
}

impl Deref for ObjTriangle {
    type Target = [ObjPointIndex; 3];

    fn deref(&self) -> &Self::Target {
        &self.indicies
    }
}
