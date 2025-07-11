use crate::primatives::Shape;
use crate::scene_tree::SceneTree;

impl SceneTree {
    pub fn flatten(&self) -> Vec<Shape> {
        let mut result = vec![];
        self.walk(&mut result);
        result
    }

    fn walk(&self, into: &mut Vec<Shape>) {
        match self {
            SceneTree::Leaf(shape) => {
                let shape = (*shape).clone();
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
    }
}
