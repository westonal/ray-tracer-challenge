use crate::intersection::{Intersect, Intersections};
use crate::primatives::IntersectableShape;
use crate::rays::Ray;
use std::ops::Deref;

pub struct FlatScene {
    chain: Vec<Chain>,
}

impl FlatScene {
    pub(crate) fn new(chain: Vec<Chain>) -> Self {
        Self { chain }
    }
}

pub enum Chain {
    BoundingVolume(IntersectableShape, usize),
    Shape(IntersectableShape),
}

impl Deref for Chain {
    type Target = IntersectableShape;

    fn deref(&self) -> &Self::Target {
        match self {
            Chain::BoundingVolume(s, _) => s,
            Chain::Shape(s) => s,
        }
    }
}

impl Deref for FlatScene {
    type Target = Vec<Chain>;

    fn deref(&self) -> &Self::Target {
        &self.chain
    }
}

impl Intersect for FlatScene {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let mut results = Intersections::default();
        let mut i = 0;
        while i < self.chain.len() {
            let item = self.chain.get(i).unwrap();
            match item {
                Chain::BoundingVolume(b, skip) => {
                    if !b.fast_hit(ray) {
                        i = i + *skip;
                    }
                }
                Chain::Shape(s) => {
                    results += s.intersect(ray);
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
    use crate::primatives::Shape;
    use crate::ray;
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::{point, vector};

    #[test]
    fn single_shape_chain() {
        let scene = FlatScene::new(vec![chain_link!(Shape::new_sphere())]);
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(2, intersections.len());
    }

    #[test]
    fn two_shape_chain() {
        let scene = FlatScene::new(vec![
            chain_link!(Shape::new_sphere()),
            chain_link!(Shape::new_sphere()),
        ]);
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_hit_no_skip_one() {
        let scene = FlatScene::new(vec![
            chain_link!(Shape::new_sphere()),
            chain_link!(Shape::new_sphere(),skip: 1),
            chain_link!(Shape::new_sphere()),
        ]);
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_missed_skip_one() {
        let scene = FlatScene::new(vec![
            chain_link!(Shape::new_sphere()),
            chain_link!(Shape::new_sphere_transformed(Matrix4x4::translation(1.1,0.,0.)),skip: 1),
            chain_link!(Shape::new_sphere()),
        ]);
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(2, intersections.len());
    }

    #[test]
    fn bounding_volume_missed_skip_one_two() {
        let scene = FlatScene::new(vec![
            chain_link!(Shape::new_sphere()),
            chain_link!(Shape::new_sphere_transformed(Matrix4x4::translation(1.1,0.,0.)),skip: 2),
            chain_link!(Shape::new_sphere()), // skipped
            chain_link!(Shape::new_sphere()), // skipped
            chain_link!(Shape::new_sphere()),
            chain_link!(Shape::new_sphere()),
        ]);
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(6, intersections.len());
    }
}

#[cfg(test)]
mod chain_build_from_tree_intersect_tests {
    use super::*;
    use crate::primatives::Shape;
    use crate::scene_tree::SceneTree;
    use crate::{ray, scene};
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::{point, vector};

    #[test]
    fn single_shape_chain() {
        let scene = scene!(
            +Shape::new_sphere();
        );
        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(2, intersections.len());
    }

    #[test]
    fn two_shape_chain() {
        let scene = scene!(
            +Shape::new_sphere();
            +Shape::new_sphere();
        );
        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_hit_no_skip_one() {
        let scene = scene!(
            +Shape::new_sphere();
            +scene!(
                bounding_volume: Shape::new_sphere();
                +Shape::new_sphere();
            );
        );

        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_missed_skip_one_due_to_bounding_volume_translation() {
        let scene = scene!(
            +Shape::new_sphere();
            +scene!(
                bounding_volume: Shape::new_sphere_transformed(Matrix4x4::translation(1.1, 0., 0.));
                +Shape::new_sphere();
            );
            +Shape::new_sphere();
        );

        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_missed_skip_one_due_to_scene_translation() {
        let scene = scene!(
            +Shape::new_sphere();
            +scene!(
                matrix: Matrix4x4::translation(1.1, 0., 0.);
                bounding_volume: Shape::new_sphere();
                +Shape::new_sphere_transformed(Matrix4x4::translation(-1.1, 0., 0.));
                +Shape::new_sphere();
            );
            +Shape::new_sphere();
        );

        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(4, intersections.len());
    }

    #[test]
    fn bounding_volume_missed_skip_one_two_due_to_bounding_volume_translation() {
        let mut scene = SceneTree::default();
        scene.add(Shape::new_sphere());
        let mut sub_scene = SceneTree::new_bounded(
            Matrix4x4::identity(),
            Shape::new_sphere_transformed(Matrix4x4::translation(1.1, 0., 0.)),
        );
        sub_scene.add(Shape::new_sphere()); // skipped
        sub_scene.add(Shape::new_sphere()); // skipped
        scene.add(sub_scene);
        scene.add(Shape::new_sphere());
        scene.add(Shape::new_sphere());

        let scene = scene.flatten();
        let intersections = scene.intersect(&ray!(point!(0, 0, -10), vector!(0, 0, 1)));
        assert_eq!(6, intersections.len());
    }
}
