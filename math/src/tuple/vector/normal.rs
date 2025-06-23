use crate::tuple::vector::Vector;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct Normal {
    vector: Vector,
}

impl Normal {
    pub fn new(vector: Vector) -> Normal {
        Self {
            vector: vector / vector.magnitude(),
        }
    }

    pub fn clone_vector(&self) -> Vector {
        self.vector.clone()
    }

    pub fn to_vector(self) -> Vector {
        self.vector
    }
}

impl Deref for Normal {
    type Target = Vector;
    fn deref(&self) -> &Self::Target {
        &self.vector
    }
}
