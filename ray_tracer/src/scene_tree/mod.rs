mod flatten;
mod manipulation;

use crate::primatives::Shape;
use math::matrix::matrix_4x4::Matrix4x4;

pub enum SceneTree {
    Leaf(Shape),
    Group {
        matrix: Matrix4x4,
        bounding_shape: Option<Shape>,
        children: Vec<SceneTree>,
    },
}

impl Default for SceneTree {
    fn default() -> Self {
        Self::Group {
            matrix: Matrix4x4::identity(),
            bounding_shape: None,
            children: Default::default(),
        }
    }
}

impl SceneTree {
    pub fn new(matrix: Matrix4x4) -> Self {
        Self::Group {
            matrix,
            bounding_shape: None,
            children: Default::default(),
        }
    }

    pub fn new_bounded(matrix: Matrix4x4, bounding_shape: Option<Shape>) -> Self {
        Self::Group {
            matrix,
            bounding_shape,
            children: Default::default(),
        }
    }
}
