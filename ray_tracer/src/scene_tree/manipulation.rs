use crate::primatives::Shape;
use crate::scene_tree::SceneTree;

impl SceneTree {
    pub fn shape_count(&self) -> usize {
        match self {
            SceneTree::Leaf(_) => 1,
            SceneTree::Group { children, .. } => children.iter().map(|a| a.shape_count()).sum(),
        }
    }

    pub fn add_tree<T: Into<SceneTree>>(&mut self, object: T) {
        match self {
            SceneTree::Leaf(_) => {
                panic!("Can't add to another leaf")
            }
            SceneTree::Group { children, .. } => {
                children.push(object.into());
            }
        }
    }

    pub fn add(&mut self, object: Shape) {
        self.add_tree(object);
    }
}

#[cfg(test)]
impl SceneTree {
    pub(crate) fn get_mut_shape(&mut self, index: usize) -> &mut Shape {
        match self {
            SceneTree::Leaf(item) => {
                if index == 0 {
                    item
                } else {
                    panic!()
                }
            }
            SceneTree::Group { children, .. } => {
                let mut countdown = index;
                for item in children.iter_mut() {
                    let branch_size = item.shape_count();
                    if branch_size > countdown {
                        return item.get_mut_shape(countdown);
                    } else {
                        countdown -= branch_size;
                    }
                }
                panic!()
            }
        }
    }
}

#[cfg(test)]
mod build_tree_tests {
    use super::*;

    #[test]
    fn new_tree() {
        let tree = SceneTree::default();
        assert_eq!(0, tree.shape_count());
    }

    #[test]
    fn add_one_leaf() {
        let mut tree = SceneTree::default();
        tree.add(Shape::new_cylinder());
        assert_eq!(1, tree.shape_count());
    }

    #[test]
    fn add_two_leafs() {
        let mut tree = SceneTree::default();
        tree.add(Shape::new_cylinder());
        tree.add(Shape::new_sphere());
        assert_eq!(2, tree.shape_count());
    }

    #[test]
    fn add_two_leafs_at_two_depths() {
        let mut tree = SceneTree::default();
        tree.add(Shape::new_cylinder());
        let mut branch = SceneTree::default();
        branch.add(Shape::new_cylinder());
        branch.add(Shape::new_sphere());
        tree.add_tree(branch);
        assert_eq!(3, tree.shape_count());
    }
}
