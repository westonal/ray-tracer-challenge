use crate::csg::CSGOperation;

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
    use crate::scene_tree::FlattenTree;

    use crate::primatives::Surface;
    use crate::scene_tree::Chain;
    use crate::{cube, scene, sphere};

    #[test]
    fn single_csg_in_a_scene() {
        let scene = scene!(+sphere!(););
        assert_chain!(
            actual: scene.flatten(),
            expect: [Surface::UnitSphere]
        );
    }

    #[test]
    fn single_csg_in_a_scene_with_bounds() {
        let scene = scene!(
            bounding_volume: cube!();
            +sphere!();
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
            +sphere!();
            +cube!();
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
            +sphere!() + cube!();
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
        let csg = sphere!() + cube!();
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
        let csg = sphere!() + (cube!() & cube!());
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
}
