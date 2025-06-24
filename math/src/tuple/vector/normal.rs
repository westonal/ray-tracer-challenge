use crate::tuple::vector::Vector;
use std::ops::{Deref, Neg};

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

impl Neg for Normal {
    type Output = Normal;

    fn neg(mut self) -> Self::Output {
        self.vector = -self.vector;
        self
    }
}

#[cfg(test)]
mod normal_negation_tests {
    use super::*;
    use crate::vector;

    #[test]
    fn invert_normal() {
        let normal = vector!(-1, 0, 0).normalize();
        assert_eq!(&vector!(-1, 0, 0), &normal.vector);
        let normal = -normal;
        assert_eq!(&vector!(1, 0, 0), &normal.vector);
    }
}
