use crate::intersection::{Intersect, Intersections};
use crate::primatives::IntersectableShape;
use crate::rays::Ray;
use crate::scene_tree::SceneTree;
use math::matrix::matrix_4x4::Matrix4x4;
use std::ops::Deref;

pub struct FlatScene {
    shapes: Vec<Chain>,
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
        &self.shapes
    }
}

impl Intersect for FlatScene {
    fn intersect(&self, ray: Ray) -> Intersections {
        let mut results = Intersections::default();
        let mut i = 0;
        while i < self.shapes.len() {
            let item = self.shapes.get(i).unwrap();
            match item {
                Chain::BoundingVolume(b, skip) => {
                    if b.intersect(ray).is_empty() {
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

impl SceneTree {
    pub fn flatten(&self) -> FlatScene {
        let mut result = vec![];
        self.walk(&mut result, Matrix4x4::identity());
        FlatScene { shapes: result }
    }

    fn walk(&self, into: &mut Vec<Chain>, tree_matrix: Matrix4x4) {
        match self {
            SceneTree::Leaf(shape) => {
                let mut shape = (*shape).clone();
                shape.matrix = tree_matrix * shape.matrix;
                into.push(Chain::Shape(shape.to_intersectable()))
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
                            bounds.matrix = matrix * bounds.matrix;
                            into.push(Chain::BoundingVolume(
                                bounds.to_intersectable(),
                                subtree.len(),
                            ));
                            // bounds.material.transparency = 0.9;
                            // into.push(Chain::Shape(
                            //     bounds.to_intersectable(),
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
    use crate::primatives::Shape;

    #[test]
    fn flatten_one() {
        let mut tree = SceneTree::default();
        tree.add(Shape::new_sphere());

        let vec = tree.flatten();
        assert_eq!(1, vec.len());
    }

    #[test]
    fn flatten_two() {
        let mut tree = SceneTree::default();
        tree.add(Shape::new_sphere());
        tree.add(Shape::new_cube());

        let vec = tree.flatten();
        assert_eq!(2, vec.len());
    }

    #[test]
    fn flatten_two_in_sub_tree() {
        let mut tree = SceneTree::default();
        tree.add(Shape::new_sphere());

        let mut branch = SceneTree::default();
        branch.add(Shape::new_cube());

        tree.add_tree(branch);

        let vec = tree.flatten();
        assert_eq!(2, vec.len());
    }

    #[test]
    fn flatten_three_in_sub_tree() {
        let mut tree = SceneTree::default();
        tree.add(Shape::new_sphere());

        let mut branch = SceneTree::default();
        branch.add(Shape::new_cube());
        branch.add(Shape::new_plane());

        tree.add_tree(branch);

        let vec = tree.flatten();
        assert_eq!(3, vec.len());
    }
}

#[cfg(test)]
mod flatten_matrix_tests {
    use super::*;
    use crate::primatives::Shape;
    use crate::transform::Transform;
    use math::degrees;

    #[test]
    fn combine_matrix_from_parent() {
        let r = Matrix4x4::shear(2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let a = Matrix4x4::scale_all(2.0);
        let b = Matrix4x4::translation(1.0, 2.0, 3.0);
        let c = Matrix4x4::rotation_x(degrees!(90));
        let d = Matrix4x4::rotation_y(degrees!(45));

        let mut root = SceneTree::new(r);
        root.add(Shape::new_sphere_transformed(a));

        let mut branch = SceneTree::new(b);
        branch.add(Shape::new_cube_transformed(c));

        root.add_tree(branch);
        root.add(Shape::new_cylinder_transformed(d));

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
