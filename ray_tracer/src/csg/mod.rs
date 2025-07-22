mod intersection;
mod build_chain;
mod filtering;

use math::matrix::matrix_4x4::Matrix4x4;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, BitXor, Sub};
use crate::primatives::{Shape, ShapeId, Surface};
use crate::scene_tree::SceneTree;
pub use filtering::TProvider;
pub use filtering::Filter;

#[macro_export]
macro_rules! csg {
    ($shape:expr) => {
        {
            let scene: $crate::scene_tree::SceneTree = $shape.into();
            scene
        }
    };
}

#[derive(Copy)]
#[derive(Clone)]
pub enum CSGOperation {
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

#[macro_export]
macro_rules! csg_sphere {
    ($(matrix: $matrix:expr)?) => {
        $crate::csg!($crate::sphere!($(matrix: $matrix)?))
    };
}

#[macro_export]
macro_rules! csg_cube {
    ($(matrix: $matrix:expr)?) => {
        $crate::csg!($crate::cube!($(matrix: $matrix)?))
    };
}

impl Debug for SceneTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SceneTree::Leaf(shape, ..) => {
                write!(f, "{:?}", shape.surface)
            }
            SceneTree::CsgLeaf(lhs, operation, rhs) => {
                write!(f, "({:?} {:?} {:?})", lhs, operation, rhs)
            },
            &SceneTree::Group { .. } => todo!()
        }
    }
}

impl Add for SceneTree {
    type Output = SceneTree;

    fn add(self, rhs: Self) -> Self::Output {
        SceneTree::CsgLeaf(self.into(), CSGOperation::Union, rhs.into())
    }
}

impl Sub for SceneTree {
    type Output = SceneTree;

    fn sub(self, rhs: Self) -> Self::Output {
        SceneTree::CsgLeaf(self.into(), CSGOperation::Difference, rhs.into())
    }
}

impl BitXor for SceneTree {
    type Output = SceneTree;

    fn bitxor(self, rhs: Self) -> Self::Output {
        SceneTree::CsgLeaf(self.into(), CSGOperation::Intersection, rhs.into())
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
