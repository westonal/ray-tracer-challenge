use crate::primatives::sphere::Sphere;
use crate::rays::Ray;
use std::ops::{AddAssign, Deref};

pub trait Intersect {
    fn intersect(&self, ray: Ray) -> Intersections;
}

pub struct Intersection<'s> {
    pub t: f32,
    // TODO, will worry about making this generic when we have more than one type
    pub sphere: &'s Sphere,
}

#[derive(Default)]
pub struct Intersections<'s>(Vec<Intersection<'s>>);

impl<'s> Intersections<'s> {
    pub fn new(p0: Vec<Intersection<'s>>) -> Intersections<'s> {
        let mut intersections = Self(p0);
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
    pub fn hit(&self) -> Option<&Intersection> {
        let mut closest_hit: Option<&Intersection> = None;
        for i in self.iter() {
            if i.t < 0. {
                continue;
            }
            match closest_hit {
                None => {
                    closest_hit = Some(i);
                }
                Some(closest_so_far) => {
                    if i.t < closest_so_far.t {
                        closest_hit = Some(i);
                    }
                }
            }
        }
        closest_hit
    }
}

impl<'s> Deref for Intersections<'s> {
    type Target = Vec<Intersection<'s>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s> Intersection<'s> {
    pub fn new(t: f32, s: &'s Sphere) -> Self {
        Self { t, sphere: s }
    }
}

#[cfg(test)]
mod sorting_tests {
    use super::*;

    #[test]
    fn intersections_are_sorted_in_create() {
        let sphere1 = Sphere::new();
        let sphere2 = Sphere::new();
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
    fn intersections_are_sorted_when_jonied() {
        let sphere1 = Sphere::new();
        let sphere2 = Sphere::new();
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
