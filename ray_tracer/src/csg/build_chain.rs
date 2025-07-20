use crate::csg::intersection::HitLocation::*;
use crate::csg::intersection::SideHit::*;
use crate::csg::{CN, CSGOperation};
use crate::material::Material;
use crate::primatives::{IntersectableShape, Shape};
use crate::scene_tree::{Chain, FlatScene, FlattenTree, SceneTree};
use crate::transform::Transform;
use math::matrix4x4;
use std::iter::Flatten;

impl FlattenTree for CN {
    fn flatten(&self) -> FlatScene {
        let mut chain = vec![];
        // TODO write left and right to the chain
        match self {
            CN::Leaf(scene) => {
                chain.append(&mut scene.flatten());
            }
            CN::Tree(lhs, op, rhs) => {
                let mut lhs = lhs.flatten();
                let mut rhs = rhs.flatten();
                chain.push(Chain::CSG(*op, lhs.len(), rhs.len()));
                chain.append(&mut lhs);
                chain.append(&mut rhs);
            }
        }
        FlatScene::new(chain)
    }
}

impl From<CN> for SceneTree {
    fn from(value: CN) -> Self {
        SceneTree::CsgLeaf(Box::new(value))
    }
}

#[cfg(test)]
macro_rules! assert_chain {
    (actual: $actual:expr, expect: [$($surface:expr$(,)?)*]) => {
        assert_eq!(vec![
               $(format!("{:?}", $surface),)*
        ], $actual
        .iter()
        .map(|f|
            match f {
                Chain::BoundingVolume(shape, skip) => {
                    format!("{:?}", ("BV", shape.surface, skip))
                }
                Chain::Shape(shape) => {
                    format!("{:?}", shape.surface)
                }
                Chain::CSG(op, skip_left, skip_right) => {
                    format!("{:?}", (op, skip_left, skip_right))
                }
            }
        ).collect::<Vec<_>>());
    };
}

#[cfg(test)]
mod filter_intersections_tests {
    use super::*;
    use crate::intersection::Intersection;
    use crate::primatives::{IntersectableShape, Shape, Surface};
    use crate::scene_tree::Chain;
    use crate::{csg_cube, csg_sphere, cube, scene, sphere};

    #[test]
    fn single_csg_intersectable() {
        let csg = csg_sphere!();
        assert_chain!(
            actual: csg.flatten(),
            expect: [Surface::UnitSphere]
        );
    }

    #[test]
    fn single_csg_in_a_scene() {
        let scene = scene!(+csg_sphere!(););
        assert_chain!(
            actual: scene.flatten(),
            expect: [Surface::UnitSphere]
        );
    }

    #[test]
    fn single_csg_in_a_scene_with_bounds() {
        let scene = scene!(
            bounding_volume: cube!();
            +csg_sphere!();
        );
        assert_chain!(
            actual: scene.flatten(),
            expect: [
                ("BV", Surface::UnitCube, 1),
                Surface::UnitSphere
            ]
        );
    }

    #[test]
    fn two_csg_in_a_scene_with_bounds() {
        let scene = scene!(
            bounding_volume: cube!();
            +csg_sphere!();
            +csg_cube!();
        );
        assert_chain!(
            actual: scene.flatten(),
            expect: [
                // TODO test is correct, walk is at fault probably
                ("BV", Surface::UnitCube, 2),
                Surface::UnitSphere,
                Surface::UnitCube,
            ]
        );
    }

    #[test]
    fn two_csg_union_in_a_scene_with_bounds() {
        let scene = scene!(
            bounding_volume: cube!();
            +csg_sphere!() + csg_cube!();
        );
        assert_chain!(
            actual: scene.flatten(),
            expect: [
                ("BV", Surface::UnitCube, 3),
                (CSGOperation::Union, 1, 1),
                Surface::UnitSphere,
                Surface::UnitCube,
            ]
        );
    }

    #[test]
    fn single_csg_union() {
        let csg = csg_sphere!() + csg_cube!();
        assert_chain!(
            actual: csg.flatten(),
            expect: [
                (CSGOperation::Union, 1, 1),
                Surface::UnitSphere,
                Surface::UnitCube,
            ]
        );
    }

    #[test]
    fn single_csg_union_and_intersection() {
        let csg = csg_sphere!() + (csg_cube!() ^ csg_cube!());
        assert_chain!(
            actual: csg.flatten(),
            expect: [
                (CSGOperation::Union, 1, 3),
                Surface::UnitSphere,
                (CSGOperation::Intersection, 1, 1),
                Surface::UnitCube,
                Surface::UnitCube,
            ]
        );
    }

    #[test]
    fn union() {
        let c = csg_sphere!() + csg_cube!();
        // todo problem, intersectons requires intersectable shapes, these are fake
        let sphere = sphere!().to_intersectable();
        let cube = cube!().to_intersectable();
        let intersections = vec![
            Intersection::new(1., &sphere),
            Intersection::new(2., &cube),
            Intersection::new(3., &sphere),
            Intersection::new(4., &cube),
        ];
    }
}
