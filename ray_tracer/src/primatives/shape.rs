use crate::material::Material;
use crate::primatives::ShapeId;
use crate::primatives::surface::Surface;
use crate::transform::Transform;
use math::matrix::matrix_4x4::Matrix4x4;
use crate::primatives::IntersectableShape;

#[derive(Debug, PartialEq, Clone)]
pub struct Shape {
    pub id: ShapeId,
    pub material: Material,
    pub(crate) matrix: Matrix4x4,
    pub(crate) surface: Surface,
}

impl Shape {
    pub(crate) fn new(object_to_world_matrix: Matrix4x4, surface: Surface) -> Self {
        Self {
            id: Default::default(),
            material: Default::default(),
            matrix: object_to_world_matrix,
            surface,
        }
    }

    pub fn to_intersectable(self) -> IntersectableShape {
        IntersectableShape {
            id: self.id,
            material: self.material,
            transform: Transform::new(self.matrix),
            surface:self.surface
        }
    }
}
