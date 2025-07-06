use crate::material::refraction::{RefractionMediumIndexes, RefractionStack};
use crate::primatives::{Shape, ShapeId};
use crate::rays::Ray;
use std::ops::{AddAssign, Deref};

pub trait Intersect {
    fn intersect(&self, ray: Ray) -> Intersections;
}

pub struct Intersection<'s> {
    pub t: f32,
    pub shape: &'s Shape,
}

impl<'s> Intersection<'s> {
    // TODO, this is quite large
    pub(crate) const EPSILON: f32 = 0.0001;
}

#[derive(Default)]
pub struct Intersections<'s>(Vec<Intersection<'s>>);

impl<'s> Intersections<'s> {
    pub fn new(vec: Vec<Intersection<'s>>) -> Intersections<'s> {
        let mut intersections = Self(vec);
        intersections.sort_by_t();
        intersections
    }
}

impl AddAssign for Intersections<'_> {
    fn add_assign(&mut self, other: Self) {
        self.0.extend(other.0.into_iter());
        self.sort_by_t();
    }
}

impl Intersections<'_> {
    fn sort_by_t(&mut self) {
        self.0.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}

impl<'s> Intersections<'s> {
    pub fn hit(&self) -> Option<(&Intersection, RefractionMediumIndexes)> {
        let mut stack = RefractionStack::new();
        for i in self.iter() {
            let refraction_indexes = stack.push(&i.shape.id, i.shape.material.refractive_index);
            if i.t < 0. {
                continue;
            }
            return Some((i, refraction_indexes));
        }
        None
    }

    pub fn hit_excluding(&self, id:&ShapeId) -> Option<(&Intersection, RefractionMediumIndexes)> {
        let mut stack = RefractionStack::new();
        for i in self.iter() {
            let refraction_indexes = stack.push(&i.shape.id, i.shape.material.refractive_index);
            if i.t < 0. || &i.shape.id == id {
                continue;
            }
            return Some((i, refraction_indexes));
        }
        None
    }
}

impl<'s> Deref for Intersections<'s> {
    type Target = Vec<Intersection<'s>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s> Intersection<'s> {
    pub fn new(t: f32, shape: &'s Shape) -> Self {
        Self { t, shape }
    }
}

#[cfg(test)]
mod sorting_tests {
    use super::*;

    #[test]
    fn intersections_are_sorted_in_create() {
        let sphere1 = Shape::new_sphere();
        let sphere2 = Shape::new_sphere();
        let intersections = Intersections::new(vec![
            Intersection::new(2., &sphere1),
            Intersection::new(1., &sphere2),
            Intersection::new(-1., &sphere2),
            Intersection::new(3., &sphere1),
        ]);
        assert_eq!(-1., intersections[0].t);
        assert_eq!(1., intersections[1].t);
        assert_eq!(2., intersections[2].t);
        assert_eq!(3., intersections[3].t);
    }

    #[test]
    fn intersections_are_sorted_when_joined() {
        let sphere1 = Shape::new_sphere();
        let sphere2 = Shape::new_sphere();
        let intersections1 = Intersections::new(vec![
            Intersection::new(1., &sphere1),
            Intersection::new(2., &sphere2),
        ]);
        let mut intersections2 = Intersections::new(vec![
            Intersection::new(3., &sphere2),
            Intersection::new(-1., &sphere1),
        ]);
        intersections2 += intersections1;
        assert_eq!(4, intersections2.len());
        assert_eq!(-1., intersections2[0].t);
        assert_eq!(1., intersections2[1].t);
        assert_eq!(2., intersections2[2].t);
        assert_eq!(3., intersections2[3].t);
    }
}

#[cfg(test)]
mod intersection_over_point_tests {
    use super::*;
    use crate::ray_first_gen;
    use math::matrix::matrix_4x4::Matrix4x4;

    #[test]
    fn the_hit_should_offset_the_point_over() {
        let shape = Shape::new_sphere_transformed(Matrix4x4::translation(0., 0., 1.));
        let i = Intersection::new(5., &shape);
        let calcs = i.to_pre_calculation(ray_first_gen!((0., 0., -5.), (0., 0., 1.)));
        assert!(calcs.surface_hit.over_point.z < -Intersection::EPSILON / 2.);
        assert!(calcs.surface_hit.point.z > calcs.surface_hit.over_point.z);
    }

    #[test]
    fn the_hit_should_offset_the_point_under() {
        let shape = Shape::new_sphere_transformed(Matrix4x4::translation(0., 0., 1.));
        let i = Intersection::new(5., &shape);
        let calcs = i.to_pre_calculation(ray_first_gen!((0., 0., -5.), (0., 0., 1.)));
        assert!(calcs.surface_hit.under_point.z > Intersection::EPSILON / 2.);
        assert!(calcs.surface_hit.point.z < calcs.surface_hit.under_point.z);
    }
}
