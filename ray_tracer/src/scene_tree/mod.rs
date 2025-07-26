mod flat_scene;
mod flatten;
mod manipulation;

use crate::csg::CSGOperation;
use crate::primatives::Shape;
pub(crate) use flat_scene::Chain;
pub use flat_scene::FlatScene;
pub use flatten::FlattenScene;
use math::matrix::matrix_4x4::Matrix4x4;

pub enum SceneTree {
    Leaf(Shape),
    CsgLeaf(Box<SceneTree>, CSGOperation, Box<SceneTree>),
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

    pub fn new_bounded(matrix: Matrix4x4, bounding_shape: Shape) -> Self {
        Self::Group {
            matrix,
            bounding_shape: Some(bounding_shape),
            children: Default::default(),
        }
    }
}

impl From<Shape> for SceneTree {
    fn from(value: Shape) -> Self {
        SceneTree::Leaf(value)
    }
}

#[macro_export]
macro_rules! scene {
    ($shape:expr) => {
        {
            let tree: $crate::scene_tree::SceneTree = $shape.into();
            tree
        }
    };

    ($(matrix: $matrix:expr;)?
     $(bounding_volume: $bounding_volume:expr;)?
     $(+$entry:expr;)*
    ) => {
        {
            let _matrix = math::matrix4x4!();
            $(let _matrix = $matrix;)?
            let mut _tree = $crate::scene_tree::SceneTree::new(_matrix);
            $(let mut _tree = $crate::scene_tree::SceneTree::new_bounded(_matrix, $bounding_volume);)?
            $(
            _tree.add($entry);
            )*
            _tree
        }
    };
}
