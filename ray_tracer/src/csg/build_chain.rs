use std::iter::Flatten;
use math::matrix4x4;
use crate::csg::intersection::HitLocation::*;
use crate::csg::intersection::SideHit::*;
use crate::csg::{CN, CSGOperation};
use crate::material::Material;
use crate::primatives::{IntersectableShape, Shape};
use crate::scene_tree::{FlatScene, FlattenTree, SceneTree};
use crate::transform::Transform;

impl FlattenTree for CN {
    fn flatten(&self) -> FlatScene {
        let mut chain = vec![];
        // TODO write left and right to the chain
        match self {
            CN::Leaf(scene) => {
                chain.append(&mut scene.flatten())
            }
            CN::Tree(_, _, _) => {}
        }
        FlatScene::new(chain)
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
                    format!("BV: {:?} :: {}", shape.surface, skip)
                }
                Chain::Shape(shape) => {
                    format!("{:?}", shape.surface)
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
    use crate::{csg_cube, csg_sphere, cube, sphere};
    use crate::scene_tree::Chain;

    #[test]
    fn single_csg_intersectable() {
        let csg = csg_sphere!();
        assert_chain!(
            actual: csg.flatten(),
            expect: [Surface::UnitSphere]
        );
    }

    #[test]
    fn single_csg_union() {
        let csg = csg_sphere!() + csg_cube!();
        assert_chain!(
            actual: csg.flatten(),
            expect: [Surface::UnitSphere]
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
