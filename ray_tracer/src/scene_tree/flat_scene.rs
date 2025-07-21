use crate::csg::{CSGOperation, Filter};
use crate::intersection::{Intersect, Intersection, Intersections};
use crate::primatives::IntersectableShape;
use crate::rays::Ray;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub struct FlatScene {
    chain: Vec<Chain>,
}

impl FlatScene {
    pub(crate) fn new(chain: Vec<Chain>) -> Self {
        Self { chain }
    }
}

#[derive(Debug)]
pub enum Chain {
    BoundingVolume(IntersectableShape, usize),
    Shape(IntersectableShape),
    CSG(CSGOperation, usize, usize),
}

#[cfg(test)]
impl Deref for Chain {
    type Target = IntersectableShape;

    fn deref(&self) -> &Self::Target {
        match self {
            Chain::BoundingVolume(s, _) => s,
            Chain::Shape(s) => s,
            Chain::CSG(_, _, _) => {
                panic!("Not permitted")
            }
        }
    }
}

impl Deref for FlatScene {
    type Target = Vec<Chain>;

    fn deref(&self) -> &Self::Target {
        &self.chain
    }
}

impl DerefMut for FlatScene{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chain
    }
}

// impl Intersect for FlatScene {
//     fn intersect(&self, ray: &Ray) -> Intersections {
//         self.chain.intersect(ray)
//     }
// }

impl Intersect for [Chain] {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let mut results = Intersections::default();
        let mut i = 0;
        while i < self.len() {
            let item = self.get(i).unwrap();
            match item {
                Chain::BoundingVolume(b, skip) => {
                    if !b.fast_hit(ray) {
                        i = i + skip;
                    }
                }
                Chain::Shape(s) => {
                    results += s.intersect(ray);
                }
                Chain::CSG(operation, lhs_length, rhs_length) => {
                    let lhs_start = i + 1;
                    let lhs = &self[lhs_start..lhs_start + lhs_length];
                    let rhs_start = lhs_start + lhs_length;
                    let rhs = &self[rhs_start..rhs_start + rhs_length];
                    let lhs_intersections = lhs.intersect(ray);
                    let rhs_intersections = rhs.intersect(ray);
                    let vec = Filter::filter::<Intersection<'_>>(
                        *operation,
                        lhs_intersections.into(),
                        rhs_intersections.into(),
                    );
                    results += Intersections::new(vec);
                    i = i + lhs_length + rhs_length;
                }
            }
            i = i + 1;
        }
        results
    }
}

macro_rules! chain_link {
    ($shape:expr) => {
        Chain::Shape($shape.to_intersectable())
    };
    ($bv:expr, skip:$skip:expr) => {
        Chain::BoundingVolume($bv.to_intersectable(), $skip)
    };
}

#[cfg(test)]
mod chain_intersect_tests {
    use super::*;

    use crate::{ray, sphere};
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::{point, vector};

    #[test]
    fn single_shape_chain() {
        let scene = FlatScene::new(vec![chain_link!(sphere!())]);
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(2, intersections.len());
    }

    #[test]
    fn two_shape_chain() {
        let scene = FlatScene::new(vec![chain_link!(sphere!()), chain_link!(sphere!())]);
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_hit_no_skip_one() {
        let scene = FlatScene::new(vec![
            chain_link!(sphere!()),
            chain_link!(sphere!(),skip: 1),
            chain_link!(sphere!()),
        ]);
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_missed_skip_one() {
        let scene = FlatScene::new(vec![
            chain_link!(sphere!()),
            chain_link!(sphere!(matrix: Matrix4x4::translation(1.1,0.,0.)),skip: 1),
            chain_link!(sphere!()),
        ]);
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(2, intersections.len());
    }

    #[test]
    fn bounding_volume_missed_skip_one_two() {
        let scene = FlatScene::new(vec![
            chain_link!(sphere!()),
            chain_link!(sphere!(matrix: Matrix4x4::translation(1.1,0.,0.)),skip: 2),
            chain_link!(sphere!()), // skipped
            chain_link!(sphere!()), // skipped
            chain_link!(sphere!()),
            chain_link!(sphere!()),
        ]);
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(6, intersections.len());
    }
}

#[cfg(test)]
mod chain_build_from_tree_intersect_tests {
    use super::*;
    use crate::scene_tree::flatten::FlattenTree;

    use crate::scene_tree::SceneTree;
    use crate::{ray, scene, sphere};
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::{point, vector};

    #[test]
    fn single_shape_chain() {
        let scene = scene!(
            +sphere!();
        );
        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(2, intersections.len());
    }

    #[test]
    fn two_shape_chain() {
        let scene = scene!(
            +sphere!();
            +sphere!();
        );
        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_hit_no_skip_one() {
        let scene = scene!(
            +sphere!();
            +scene!(
                bounding_volume: sphere!();
                +sphere!();
            );
        );

        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_missed_skip_one_due_to_bounding_volume_translation() {
        let scene = scene!(
            +sphere!();
            +scene!(
                bounding_volume: sphere!(matrix: Matrix4x4::translation(1.1, 0., 0.));
                +sphere!();
            );
            +sphere!();
        );

        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_missed_skip_one_due_to_scene_translation() {
        let scene = scene!(
            +sphere!();
            +scene!(
                matrix: Matrix4x4::translation(1.1, 0., 0.);
                bounding_volume: sphere!();
                +sphere!(matrix: Matrix4x4::translation(-1.1, 0., 0.));
                +sphere!();
            );
            +sphere!();
        );

        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_missed_skip_one_two_due_to_bounding_volume_translation() {
        let mut scene = SceneTree::default();
        scene.add(sphere!());
        let mut sub_scene = SceneTree::new_bounded(
            Matrix4x4::identity(),
            sphere!(matrix: Matrix4x4::translation(1.1, 0., 0.)),
        );
        sub_scene.add(sphere!()); // skipped
        sub_scene.add(sphere!()); // skipped
        scene.add(sub_scene);
        scene.add(sphere!());
        scene.add(sphere!());

        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(6, intersections.len());
    }
}
