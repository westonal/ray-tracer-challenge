use crate::rays::Ray;
use math::tuple::point::Point;
use math::tuple::vector::Vector;
use math::vector;

#[derive(PartialEq, Debug)]
pub(crate) enum Surface {
    UnitSphere,
    PlaneXZ,
}

impl Surface {
    pub(crate) fn intersect(&self, ray: Ray) -> Vec<f32> {
        let sphere_to_ray = ray.origin - Point::origin();
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return Default::default();
        }
        let mut result = Vec::with_capacity(2);
        let a2 = 2. * a;
        if discriminant == 0. {
            result.push(-b / a2);
        } else {
            let discriminant_sqrt = discriminant.sqrt();
            result.push((-b - discriminant_sqrt) / a2);
            result.push((-b + discriminant_sqrt) / a2);
        }
        result
    }
    pub(crate) fn normal_at(&self, object_point: Point) -> Vector {
        match self {
            Surface::UnitSphere => object_point - Point::origin(),
            Surface::PlaneXZ => vector!(0, 1, 0),
        }
    }
}
