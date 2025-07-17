use crate::ObjTriangle;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct Group {
    pub name: Option<String>,
    pub triangles: Vec<ObjTriangle>,
}

impl Deref for Group {
    type Target = Vec<ObjTriangle>;

    fn deref(&self) -> &Self::Target {
        &self.triangles
    }
}

impl Group {
    pub fn default_group(triangles: Vec<ObjTriangle>) -> Self {
        Self {
            name: None,
            triangles,
        }
    }

    pub fn new(name: Option<String>, triangles: Vec<ObjTriangle>) -> Self {
        Self { name, triangles }
    }
}
