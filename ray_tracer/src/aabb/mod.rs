mod axis;

use crate::aabb::axis::AABBAxis;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::point::Point;
use math::{matrix4x4, point};
use std::ops::AddAssign;

#[derive(Debug, Default, Copy, Clone)]
pub struct AABB {
    x: AABBAxis,
    y: AABBAxis,
    z: AABBAxis,
}

impl AABB {
    pub fn min_point(&self) -> Option<Point> {
        if self.is_empty() {
            return None;
        }
        Some(point!(self.x.min, self.y.min, self.z.min))
    }

    pub fn max_point(&self) -> Option<Point> {
        if self.is_empty() {
            return None;
        }
        Some(point!(self.x.max, self.y.max, self.z.max))
    }

    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }

    pub fn to_bounding_range(&self) -> Option<Matrix4x4> {
        if self.is_empty() {
            return None;
        }
        let resized = self.clone_with_min_width(0.1);
        Some(matrix4x4!(
            translation(resized.x.min, resized.y.min, resized.z.min)
            scale(resized.x.width(), resized.y.width(), resized.z.width())
            translation(0.5, 0.5, 0.5)
            scale_all(0.5)
        ))
    }

    fn clone_with_min_width(&self, min_width: f32) -> AABB {
        let mut aabb = self.clone();
        aabb.x.ensure_some_width(min_width);
        aabb.y.ensure_some_width(min_width);
        aabb.z.ensure_some_width(min_width);
        aabb
    }
}

impl AABB {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AABB {
    pub fn push_point(&mut self, point: &Point) {
        self.x.push(point.x);
        self.y.push(point.y);
        self.z.push(point.z);
    }

    pub fn push_points(&mut self, points: &[Point]) {
        for p in points {
            self.push_point(p);
        }
    }
}

impl AABBPushable for AABB {
    fn extreme_points(&self) -> Vec<Point> {
        vec![self.min_point(), self.max_point()]
            .into_iter()
            .flatten()
            .collect()
    }
}

impl AABBPushable for &[Point] {
    fn extreme_points(&self) -> Vec<Point> {
        self.to_vec()
    }
}

pub trait AABBPushable {
    /// Output some extreme points which can be used to expand an [AABB].
    fn extreme_points(&self) -> Vec<Point>;
}

impl<T: AABBPushable> AddAssign<T> for AABB {
    fn add_assign(&mut self, rhs: T) {
        self.push_points(&rhs.extreme_points())
    }
}

#[cfg(test)]
mod aabb_tests {
    use super::*;

    #[test]
    fn initially_empty() {
        let empty_aabb = AABB::new();
        let matrix = empty_aabb.to_bounding_range();
        assert_eq!(None, matrix);
        assert_eq!(None, empty_aabb.min_point());
        assert_eq!(None, empty_aabb.max_point());
    }

    #[test]
    fn push_one_point_min_and_max_points_available() {
        let mut aabb = AABB::new();
        aabb.push_point(&point!(1, 2, 3));
        assert_eq!(Some(point!(1, 2, 3)), aabb.min_point());
        assert_eq!(Some(point!(1, 2, 3)), aabb.max_point());
    }

    #[test]
    fn push_two_points_that_define_a_box() {
        let mut aabb = AABB::new();
        aabb.push_point(&point!(0, 0, 0));
        aabb.push_point(&point!(1, 1, 1));
        assert_eq!(
            matrix4x4!(
                [0.5,   0,   0, 0.5]
                [  0, 0.5,   0, 0.5]
                [  0,   0, 0.5, 0.5]
                [  0,   0,   0,   1]
            ),
            aabb.to_bounding_range().unwrap()
        );
    }

    #[test]
    fn push_three_points_separately_that_define_a_box() {
        let mut aabb = AABB::new();
        aabb.push_point(&point!(1, 0, 0));
        aabb.push_point(&point!(0, 1, 0));
        aabb.push_point(&point!(0, 0, 1));
        assert_eq!(
            matrix4x4!(
                [0.5,   0,   0, 0.5]
                [  0, 0.5,   0, 0.5]
                [  0,   0, 0.5, 0.5]
                [  0,   0,   0,   1]
            ),
            aabb.to_bounding_range().unwrap()
        );
    }

    #[test]
    fn push_three_points_in_vec_that_define_a_box() {
        let mut aabb = AABB::new();
        aabb.push_points(&vec![point!(1, 0, 0), point!(0, 1, 0), point!(0, 0, 1)]);
        assert_eq!(
            matrix4x4!(
                [0.5,   0,   0, 0.5]
                [  0, 0.5,   0, 0.5]
                [  0,   0, 0.5, 0.5]
                [  0,   0,   0,   1]
            ),
            aabb.to_bounding_range().unwrap()
        );
    }

    #[test]
    fn push_two_larger_points_that_define_a_box() {
        let mut aabb = AABB::new();
        aabb.push_point(&point!(0, 0, 0));
        aabb.push_point(&point!(10, 10, 10));
        assert_eq!(
            matrix4x4!(
                [5, 0, 0, 5]
                [0, 5, 0, 5]
                [0, 0, 5, 5]
                [0, 0, 0, 1]
            ),
            aabb.to_bounding_range().unwrap()
        );
    }

    #[test]
    fn min_point_max_point() {
        let mut aabb = AABB::new();
        aabb.push_point(&point!(1, -15, 0));
        aabb.push_point(&point!(-2, -1, -5));
        aabb.push_point(&point!(-4, -5, 3));
        assert_eq!(Some(point!(-4, -15, -5)), aabb.min_point());
        assert_eq!(Some(point!(1, -1, 3)), aabb.max_point());
    }

    #[test]
    fn aabb_add_assign_when_both_not_empty() {
        let mut aabb1 = AABB::new();
        aabb1.push_point(&point!(-1, -1, -1));
        let mut aabb2 = AABB::new();
        aabb2.push_point(&point!(1, 1, 1));
        aabb1 += aabb2;
        assert_eq!(Some(point!(-1, -1, -1)), aabb1.min_point());
        assert_eq!(Some(point!(1, 1, 1)), aabb1.max_point());
    }

    #[test]
    fn aabb_add_assign() {
        let mut aabb1 = AABB::new();
        aabb1.push_point(&point!(-1, -1, -1));
        let mut aabb2 = AABB::new();
        aabb2.push_point(&point!(1, 1, 1));
        aabb1 += aabb2;
        assert_eq!(Some(point!(-1, -1, -1)), aabb1.min_point());
        assert_eq!(Some(point!(1, 1, 1)), aabb1.max_point());
    }

    #[test]
    fn aabb_add_assign_when_one_empty() {
        let mut aabb1 = AABB::new();
        aabb1.push_point(&point!(-1, -1, -1));
        aabb1 += AABB::new();
        assert_eq!(Some(point!(-1, -1, -1)), aabb1.min_point());
        assert_eq!(Some(point!(-1, -1, -1)), aabb1.max_point());
    }

    #[test]
    fn aabb_add_assign_when_destination_empty() {
        let mut aabb1 = AABB::new();
        let mut aabb2 = AABB::new();
        aabb2.push_point(&point!(-1, -1, -1));
        aabb1 += aabb2;
        assert_eq!(Some(point!(-1, -1, -1)), aabb1.min_point());
        assert_eq!(Some(point!(-1, -1, -1)), aabb1.max_point());
    }
}
