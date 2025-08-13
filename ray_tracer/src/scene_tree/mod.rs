mod aabb;
mod auto_bounding_volume;
mod flat_scene;
mod flatten;
mod manipulation;

use crate::csg::CSGOperation;
use crate::cube;
use crate::lighting::PointLight;
use crate::material::Material;
use crate::primatives::Shape;
pub(crate) use flat_scene::Chain;
pub use flat_scene::FlatScene;
pub use flatten::FlattenScene;
use math::matrix::matrix_4x4::Matrix4x4;
use std::sync::LazyLock;

#[derive(Clone)]
pub enum SceneTree {
    Light(PointLight),
    Leaf(Shape),
    CsgLeaf(Box<SceneTree>, CSGOperation, Box<SceneTree>),
    Group {
        matrix: Matrix4x4,
        material_override: Option<Material>,
        bounding_shape: Option<Shape>,
        children: Vec<SceneTree>,
    },
}

impl Default for SceneTree {
    fn default() -> Self {
        Self::Group {
            matrix: Matrix4x4::identity(),
            material_override: None,
            bounding_shape: None,
            children: Default::default(),
        }
    }
}

impl SceneTree {
    pub fn new(matrix: Matrix4x4) -> Self {
        Self::Group {
            matrix,
            material_override: None,
            bounding_shape: None,
            children: Default::default(),
        }
    }

    pub fn new_bounded(
        matrix: Matrix4x4,
        bounding_shape: Option<Shape>,
        material_override: Option<Material>,
    ) -> Self {
        Self::Group {
            matrix,
            material_override,
            bounding_shape,
            children: Default::default(),
        }
    }

    pub fn is_not_empty(&self) -> bool {
        match self {
            SceneTree::Light(_) => true,
            SceneTree::Leaf(_) => true,
            SceneTree::CsgLeaf(_, _, _) => true,
            SceneTree::Group { children, .. } => children.iter().any(|c| c.is_not_empty()),
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.is_not_empty()
    }
}

impl From<Shape> for SceneTree {
    fn from(value: Shape) -> Self {
        SceneTree::Leaf(value)
    }
}

impl From<PointLight> for SceneTree {
    fn from(value: PointLight) -> Self {
        SceneTree::Light(value)
    }
}

pub static AUTO_CUBE_BOUNDING_VOLUME: LazyLock<Shape> = LazyLock::new(|| cube!());

/// Auto Bounding Volume
#[macro_export]
macro_rules! auto {
    () => {
        $crate::scene_tree::AUTO_CUBE_BOUNDING_VOLUME.clone()
    };
}

#[macro_export]
macro_rules! scene {
    () => {
        $crate::scene_tree::SceneTree::new(math::matrix4x4!())
    };

    ($shape:expr) => {
        {
            let tree: $crate::scene_tree::SceneTree = $shape.into();
            tree
        }
    };

    ($(iff: $iff:expr;)?
     $(matrix: $matrix:expr;)?
     $(material_override: $material_override:expr;)?
     $(bounding_volume: $bounding_volume:expr;)?
     $(+$entry:expr;)*
    ) => {
        {
            let mut _condition = true;
            $(_condition = $iff;)?
            if _condition {
                let _matrix = math::matrix4x4!();
                let _bounding_volume: Option<$crate::primatives::Shape> = None;
                let _material_override: Option<$crate::material::Material> = None;
                $(let _matrix = $matrix;)?
                $(let _bounding_volume = Some($bounding_volume);)?
                $(let _material_override = Some($material_override);)?
                let mut _tree = $crate::scene_tree::SceneTree::new_bounded(
                    _matrix,
                    _bounding_volume,
                    _material_override
                );
                $(
                _tree.add($entry);
                )*
                _tree
            } else {
                scene!()
            }
        }
    };
}
