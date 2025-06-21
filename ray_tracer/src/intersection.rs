use crate::primatives::sphere::Sphere;
use crate::rays::Ray;
use std::ops::Deref;

pub trait Intersect {
    fn intersect(&self, ray: Ray) -> Intersections;
}

pub struct Intersection<'s> {
    pub t: f32,
    // TODO, will worry about making this generic when we have more than one type
    pub sphere: &'s Sphere,
}

pub struct Intersections<'s>(pub Vec<Intersection<'s>>);

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
