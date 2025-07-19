use crate::material::Material;
use crate::primatives::IntersectableShape;
use crate::primatives::ShapeId;
use crate::primatives::surface::Surface;
use crate::transform::Transform;
use math::matrix::matrix_4x4::Matrix4x4;

#[derive(Debug, PartialEq, Clone)]
pub struct Shape {
    pub id: ShapeId,
    pub material: Material,
    pub(crate) matrix: Matrix4x4,
    pub(crate) surface: Surface,
}

impl Shape {
    pub fn new(object_to_world_matrix: Matrix4x4, surface: Surface) -> Self {
        Self {
            id: Default::default(),
            material: Default::default(),
            matrix: object_to_world_matrix,
            surface,
        }
    }

    pub fn to_intersectable(self) -> IntersectableShape {
        self.into()
    }
}

impl From<Shape> for IntersectableShape {
    fn from(value: Shape) -> Self {
        IntersectableShape {
            id: value.id,
            material: value.material,
            transform: Transform::new(value.matrix),
            surface: value.surface,
        }
    }
}
