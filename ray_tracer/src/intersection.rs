use crate::material::refraction::{RefractionMediumIndexes, RefractionStack};
use crate::primatives::IntersectableShape;
use crate::primatives::ShapeId;
use crate::rays::Ray;
use std::ops::{AddAssign, Deref};

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Intersections;

    fn fast_hit(&self, ray: &Ray) -> bool {
        self.intersect(ray).hit().is_some()
    }
}

#[derive(Copy, Clone)]
pub struct UV {
    pub u: f32,
    pub v: f32,
}

#[derive(Copy, Clone)]
pub struct FUV {
    pub t: f32,
    pub uv: Option<UV>,
}

impl FUV {
    pub(crate) fn new(f: f32, uv: UV) -> Self {
        Self { t: f, uv: Some(uv) }
    }

    pub(crate) fn justF(f: f32) -> Self {
        Self { t: f, uv: None }
    }
}

impl UV {
    pub fn new(u: f32, v: f32) -> Self {
        Self { u, v }
    }
}

pub struct Intersection<'s> {
    pub fuv: FUV,
    pub shape: &'s IntersectableShape,
}

impl Deref for Intersection<'_> {
    type Target = FUV;

    fn deref(&self) -> &Self::Target {
        &self.fuv
    }
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

    pub fn hit_excluding(&self, id: &ShapeId) -> Option<(&Intersection, RefractionMediumIndexes)> {
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
    pub fn new_fuv(fuv: FUV, shape: &'s IntersectableShape) -> Self {
        Self { fuv, shape }
    }

    pub fn new(t: f32, shape: &'s IntersectableShape) -> Self {
        Self {
            fuv: FUV { t, uv: None },
            shape,
        }
    }
}

#[cfg(test)]
mod sorting_tests {
    use super::*;
    use crate::primatives::Shape;

    #[test]
    fn intersections_are_sorted_in_create() {
        let sphere1 = Shape::new_sphere().to_intersectable();
        let sphere2 = Shape::new_sphere().to_intersectable();
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
        let sphere1 = Shape::new_sphere().to_intersectable();
        let sphere2 = Shape::new_sphere().to_intersectable();
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
