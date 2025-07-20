use crate::scene_tree::SceneTree;
use crate::scene_tree::flat_scene::{Chain, FlatScene};
use math::matrix::matrix_4x4::Matrix4x4;
use math::matrix4x4;

pub trait FlattenTree {
    fn flatten(&self) -> FlatScene;
}

impl FlattenTree for SceneTree {
    fn flatten(&self) -> FlatScene {
        let mut chain = vec![];
        self.walk(&mut chain, matrix4x4!());
        FlatScene::new(chain)
    }
}

impl SceneTree {
    fn walk(&self, into: &mut Vec<Chain>, tree_matrix: Matrix4x4) {
        match self {
            SceneTree::Leaf(shape) => {
                let mut shape = (*shape).clone();
                shape.matrix = tree_matrix * shape.matrix;
                into.push(Chain::Shape(shape.to_intersectable()))
            }
            SceneTree::CsgLeaf(csg) => {
                into.append(&mut csg.flatten())
            }
            SceneTree::Group {
                children,
                matrix,
                bounding_shape,
            } => {
                let matrix = tree_matrix * *matrix;

                match bounding_shape {
                    None => {
                        for child in children {
                            child.walk(into, matrix);
                        }
                    }
                    Some(bounds) => {
                        for child in children {
                            let mut subtree = vec![];
                            child.walk(&mut subtree, matrix);
                            let mut bounds = bounds.clone();
                            // let mut bounds2 = bounds.clone();
                            bounds.matrix = matrix * bounds.matrix;
                            into.push(Chain::BoundingVolume(
                                bounds.to_intersectable(),
                                subtree.len(), // + 1,
                            ));
                            // bounds2.material.transparency = 0.9;
                            // into.push(Chain::Shape(
                            //     bounds2.to_intersectable(),
                            //     //subtree.len(),
                            // ));
                            into.append(&mut subtree);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod flatten_tests {
    use super::*;

    use crate::{cube, plane, sphere};

    #[test]
    fn flatten_one() {
        let mut tree = SceneTree::default();
        tree.add(sphere!());

        let vec = tree.flatten();
        assert_eq!(1, vec.len());
    }

    #[test]
    fn flatten_two() {
        let mut tree = SceneTree::default();
        tree.add(sphere!());
        tree.add(cube!());

        let vec = tree.flatten();
        assert_eq!(2, vec.len());
    }

    #[test]
    fn flatten_two_in_sub_tree() {
        let mut tree = SceneTree::default();
        tree.add(sphere!());

        let mut branch = SceneTree::default();
        branch.add(cube!());

        tree.add(branch);

        let vec = tree.flatten();
        assert_eq!(2, vec.len());
    }

    #[test]
    fn flatten_three_in_sub_tree() {
        let mut tree = SceneTree::default();
        tree.add(sphere!());

        let mut branch = SceneTree::default();
        branch.add(cube!());
        branch.add(plane!());

        tree.add(branch);

        let vec = tree.flatten();
        assert_eq!(3, vec.len());
    }
}

#[cfg(test)]
mod flatten_matrix_tests {
    use super::*;

    use crate::transform::Transform;
    use crate::{cube, cylinder, sphere};
    use math::{degrees, matrix4x4};

    #[test]
    fn combine_matrix_from_parent() {
        let r = matrix4x4!(shear(2.0, 3.0, 4.0, 5.0, 6.0, 7.0));
        let a = matrix4x4!(scale_all(2.0));
        let b = matrix4x4!(translation(1.0, 2.0, 3.0));
        let c = matrix4x4!(rotation_x(degrees!(90)));
        let d = matrix4x4!(rotation_y(degrees!(45)));

        let mut root = SceneTree::new(r);
        root.add(sphere!(matrix: a));

        let mut branch = SceneTree::new(b);
        branch.add(cube!(matrix: c));

        root.add(branch);
        root.add(cylinder!(matrix: d));

        let vec = root.flatten();

        // root (r)
        //   - tree (i)
        //       - sphere (a) => r * a
        //   - tree (b)
        //       - cube (c) => r * c * b
        //   - cylinder (d) => r * d
        //

        assert_eq!(3, vec.len());
        assert_eq!(Transform::new(r * a), vec.get(0).unwrap().transform);
        assert_eq!(Transform::new(r * b * c), vec.get(1).unwrap().transform);
        assert_eq!(Transform::new(r * d), vec.get(2).unwrap().transform);
    }
}
