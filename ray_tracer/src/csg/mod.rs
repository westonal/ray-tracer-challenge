mod build_chain;
mod filtering;
mod intersection;

use crate::primatives::Shape;
use crate::scene;
use crate::scene_tree::SceneTree;
pub use filtering::Filter;
pub use filtering::TProvider;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, BitAnd, Sub};

#[derive(Copy, Clone)]
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

impl Debug for SceneTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SceneTree::Light(point_light, ..) => {
                write!(
                    f,
                    "PointLight({:?}, {:?})",
                    point_light.position, point_light.color
                )
            }
            SceneTree::Leaf(shape, ..) => {
                write!(f, "{:?}", shape.surface)
            }
            SceneTree::CsgLeaf(lhs, operation, rhs) => {
                write!(f, "({:?} {:?} {:?})", lhs, operation, rhs)
            }
            SceneTree::Group {
                bounding_shape,
                children,
                ..
            } => {
                write!(f, "[bv:{:?}; {:?}]", bounding_shape, children.len())
            }
        }
    }
}

impl<T: Into<SceneTree>> Add<T> for Shape {
    type Output = SceneTree;

    fn add(self, rhs: T) -> Self::Output {
        scene!(self) + rhs
    }
}

impl<T: Into<SceneTree>> Add<T> for SceneTree {
    type Output = SceneTree;

    fn add(self, rhs: T) -> Self::Output {
        SceneTree::CsgLeaf(self.into(), CSGOperation::Union, rhs.into().into())
    }
}

impl<T: Into<SceneTree>> Sub<T> for Shape {
    type Output = SceneTree;

    fn sub(self, rhs: T) -> Self::Output {
        scene!(self) - rhs
    }
}

impl<T: Into<SceneTree>> Sub<T> for SceneTree {
    type Output = SceneTree;

    fn sub(self, rhs: T) -> Self::Output {
        SceneTree::CsgLeaf(self.into(), CSGOperation::Difference, rhs.into().into())
    }
}

impl<T: Into<SceneTree>> BitAnd<T> for Shape {
    type Output = SceneTree;

    fn bitand(self, rhs: T) -> Self::Output {
        scene!(self) & rhs
    }
}

impl<T: Into<SceneTree>> BitAnd<T> for SceneTree {
    type Output = SceneTree;

    fn bitand(self, rhs: T) -> Self::Output {
        SceneTree::CsgLeaf(self.into(), CSGOperation::Intersection, rhs.into().into())
    }
}

#[cfg(test)]
mod constructive_solid_geometry_dsl_tests {

    use crate::{cube, scene, sphere};

    #[test]
    fn union_scene_and_scene() {
        let union = scene!(sphere!()) + scene!(cube!());
        assert_eq!("(UnitSphere ∪ UnitCube)", format!("{:?}", union));
    }

    #[test]
    fn union_scene_and_shape() {
        let union = scene!(sphere!()) + cube!();
        assert_eq!("(UnitSphere ∪ UnitCube)", format!("{:?}", union));
    }

    #[test]
    fn union_shape_and_scene() {
        let union = sphere!() + scene!(cube!());
        assert_eq!("(UnitSphere ∪ UnitCube)", format!("{:?}", union));
    }

    #[test]
    fn union_shape_and_shape() {
        let union = sphere!() + cube!();
        assert_eq!("(UnitSphere ∪ UnitCube)", format!("{:?}", union));
    }

    #[test]
    fn intersection_scene_and_scene() {
        let intersection = scene!(sphere!()) & scene!(cube!());
        assert_eq!("(UnitSphere ∩ UnitCube)", format!("{:?}", intersection));
    }

    #[test]
    fn intersection_scene_and_shape() {
        let intersection = scene!(sphere!()) & cube!();
        assert_eq!("(UnitSphere ∩ UnitCube)", format!("{:?}", intersection));
    }

    #[test]
    fn intersection_shape_and_scene() {
        let intersection = sphere!() & scene!(cube!());
        assert_eq!("(UnitSphere ∩ UnitCube)", format!("{:?}", intersection));
    }

    #[test]
    fn intersection_shape_and_shape() {
        let intersection = sphere!() & cube!();
        assert_eq!("(UnitSphere ∩ UnitCube)", format!("{:?}", intersection));
    }

    #[test]
    fn difference_scene_and_scene() {
        let difference = scene!(sphere!()) - scene!(cube!());
        assert_eq!("(UnitSphere - UnitCube)", format!("{:?}", difference));
    }

    #[test]
    fn difference_shape_and_scene() {
        let difference = sphere!() - scene!(cube!());
        assert_eq!("(UnitSphere - UnitCube)", format!("{:?}", difference));
    }

    #[test]
    fn difference_scene_and_shape() {
        let difference = scene!(sphere!()) - cube!();
        assert_eq!("(UnitSphere - UnitCube)", format!("{:?}", difference));
    }

    #[test]
    fn difference_shape_and_shape() {
        let difference = sphere!() - cube!();
        assert_eq!("(UnitSphere - UnitCube)", format!("{:?}", difference));
    }

    #[test]
    fn complex() {
        let csg_tree = sphere!() - (cube!() + cube!()) & sphere!();
        assert_eq!(
            "((UnitSphere - (UnitCube ∪ UnitCube)) ∩ UnitSphere)",
            format!("{:?}", csg_tree)
        )
    }
}
