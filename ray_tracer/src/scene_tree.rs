use crate::primatives::Shape;
use crate::transform::Transform;

pub enum SceneTree {
    Leaf(Shape),
    Group {
        transform: Transform,
        bounding_shape: Option<Shape>,
        children: Vec<SceneTree>,
    },
}

impl Default for SceneTree {
    fn default() -> Self {
        Self::Group {
            transform: Transform::identity(),
            bounding_shape: None,
            children: Default::default(),
        }
    }
}

impl SceneTree {
    pub fn new(transform: Transform) -> Self {
        Self::Group {
            transform,
            bounding_shape: None,
            children: Default::default(),
        }
    }

    pub fn new_bounded(transform: Transform, bounding_shape: Option<Shape>) -> Self {
        Self::Group {
            transform,
            bounding_shape,
            children: Default::default(),
        }
    }
}
