use math::matrix::matrix_4x4::Matrix4x4;
use crate::primatives::{Shape, Shape2};
use crate::scene_tree::SceneTree;
use crate::transform::Transform;

impl SceneTree {
    pub fn flatten(&self) -> Vec<Shape2> {
        let mut result = vec![];
        self.walk(&mut result);
        result
    }

    fn walk(&self, into: &mut Vec<Shape2>) {
        match self {
            SceneTree::Leaf(shape) => {
                let mut shape = (*shape).clone().to_shape2();
                // shape.transform = Transform::identity();// * Matrix4x4::scale_all(2.);
                into.push(shape)
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

        //tree.flatten()
        todo!()
    }
}
