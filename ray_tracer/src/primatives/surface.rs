use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;

#[derive(PartialEq, Debug)]
pub(crate) enum Surface {
    UnitSphere,
    PlaneXZ,
}

impl Surface {
    pub(crate) fn intersect(&self, ray: Ray) -> Vec<f32> {
        match self {
            Surface::UnitSphere => self.sphere_intersect(ray),
            Surface::PlaneXZ => self.plane_intersect(ray),
        }
    }

    pub(crate) fn normal_at(&self, object_point: Point) -> Vector {
        match self {
            Surface::UnitSphere => self.sphere_normal_at(object_point),
            Surface::PlaneXZ => self.plane_normal_at(object_point),
        }
    }
}
