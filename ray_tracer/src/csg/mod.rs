mod intersection;

use math::matrix::matrix_4x4::Matrix4x4;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, BitXor, Sub};
use crate::primatives::{ShapeId, Surface};

#[macro_export]
macro_rules! csg {
    (surface: $surface:expr; $(matrix: $matrix:expr;)?) => {
        {
            let mut _m = math::matrix::matrix_4x4::Matrix4x4::identity();
            $(_m = $matrix;)?
            $crate::csg::CN::Leaf($surface, $crate::primatives::ShapeId::default(), _m)
        }
    };
}

enum CSGOperation {
    Union,
    Intersection,
    Difference,
}

impl Debug for CSGOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CSGOperation::Union => {
                write!(f, "∪")
            }
            CSGOperation::Intersection => {
                write!(f, "∩")
            }
            CSGOperation::Difference => {
                write!(f, "-")
            }
        }
    }
}

enum CN {
    Leaf(Surface, ShapeId, Matrix4x4),
    Tree(Box<CN>, CSGOperation, Box<CN>),
}

#[macro_export]
macro_rules! csg_sphere {
    ($(matrix: $matrix:expr)?) => {
        $crate::csg!(surface: $crate::primatives::Surface::UnitSphere; $(matrix: $matrix;)?)
    };
}

#[macro_export]
macro_rules! csg_cube {
    ($(matrix: $matrix:expr)?) => {
        $crate::csg!(surface: $crate::primatives::Surface::UnitCube; $(matrix: $matrix;)?)
    };
}

impl Debug for CN {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CN::Leaf(ba, ..) => {
                write!(f, "{:?}", ba)
            }
            CN::Tree(lhs, operation, rhs) => {
                write!(f, "({:?} {:?} {:?})", lhs, operation, rhs)
            }
        }
    }
}

impl Add for CN {
    type Output = CN;

    fn add(self, rhs: Self) -> Self::Output {
        CN::Tree(self.into(), CSGOperation::Union, rhs.into())
    }
}

impl Sub for CN {
    type Output = CN;

    fn sub(self, rhs: Self) -> Self::Output {
        CN::Tree(self.into(), CSGOperation::Difference, rhs.into())
    }
}

impl BitXor for CN {
    type Output = CN;

    fn bitxor(self, rhs: Self) -> Self::Output {
        CN::Tree(self.into(), CSGOperation::Intersection, rhs.into())
    }
}

#[cfg(test)]
mod constructive_solid_geometry_dsl_tests {
    use super::*;

    #[test]
    fn union() {
        let union = csg_sphere!() + csg_cube!();
        assert_eq!("(UnitSphere ∪ UnitCube)", format!("{:?}", union))
    }

    #[test]
    fn intersection() {
        let intersection = csg_sphere!() ^ csg_cube!();
        assert_eq!("(UnitSphere ∩ UnitCube)", format!("{:?}", intersection))
    }

    #[test]
    fn difference() {
        let difference = csg_sphere!() - csg_cube!();
        assert_eq!("(UnitSphere - UnitCube)", format!("{:?}", difference))
    }

    #[test]
    fn complex() {
        let csg_tree = csg_sphere!() - (csg_cube!() + csg_cube!()) ^ csg_sphere!();
        assert_eq!(
            "((UnitSphere - (UnitCube ∪ UnitCube)) ∩ UnitSphere)",
            format!("{:?}", csg_tree)
        )
    }
}
