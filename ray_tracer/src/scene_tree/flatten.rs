use crate::primatives::IntersectableShape;
use crate::scene_tree::SceneTree;
use math::matrix::matrix_4x4::Matrix4x4;

impl SceneTree {
    pub fn flatten(&self) -> Vec<IntersectableShape> {
        let mut result = vec![];
        self.walk(&mut result);
        result
    }

    fn walk(&self, into: &mut Vec<IntersectableShape>) {
        match self {
            SceneTree::Leaf(shape) => {
                let mut shape = (*shape).clone();
                // TODO apply group matrix here
                shape.matrix = shape.matrix * Matrix4x4::identity();
                into.push(shape.to_intersectable())
            }
            SceneTree::Group { children, .. } => {
                for child in children {
                    child.walk(into)
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
