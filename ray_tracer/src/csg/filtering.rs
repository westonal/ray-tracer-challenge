use crate::csg::CSGOperation;
use crate::csg::intersection::{HitLocation, SideHit, intersection_allowed};
use crate::primatives::ShapeId;

pub struct Filter {}

pub trait TProvider {
    fn t(&self) -> f32;
}

impl Filter {
    pub fn filter<T: TProvider>(
        csg_operation: CSGOperation,
        lhs: Vec<T>,
        rhs: Vec<T>,
    ) -> Vec<T> {
        let mut annotated_lhs: Vec<(T, SideHit)> =
            lhs.into_iter().map(|f| (f, SideHit::Left)).collect();
        let mut annotated_rhs: Vec<(T, SideHit)> =
            rhs.into_iter().map(|f| (f, SideHit::Right)).collect();
        let mut result: Vec<(T, SideHit)> =
            Vec::with_capacity(annotated_lhs.len() + annotated_rhs.len());
        result.append(&mut annotated_lhs);
        result.append(&mut annotated_rhs);
        result.sort_by(|(a, _), (b, _)| a.t().partial_cmp(&b.t()).unwrap());

        let mut left_location = HitLocation::Outside;
        let mut right_location = HitLocation::Outside;

        let mut filtered: Vec<T> = Vec::with_capacity(result.len());
        // consider each intersection in distance order
        for (t, side) in result {
            match side {
                SideHit::Left => left_location = !left_location,
                SideHit::Right => right_location = !right_location,
            }
            if intersection_allowed(
                csg_operation,
                side,
                left_location,
                right_location,
            ) {
                filtered.push(t);
            }
        }
        filtered
    }
}

#[cfg(test)]
mod filter_tests {
    use super::*;

    #[test]
    fn a() {
        
        
        
    }
}
