use crate::csg::CSGOperation;
use crate::csg::intersection::{HitLocation, SideHit, intersection_allowed};

pub struct Filter {}

pub trait TProvider {
    fn t(&self) -> f32;
}

impl Filter {
    pub fn filter<T: TProvider>(csg_operation: CSGOperation, lhs: Vec<T>, rhs: Vec<T>) -> Vec<T> {
        let mut lhs: Vec<T> = lhs.into_iter().rev().collect();
        let mut rhs: Vec<T> = rhs.into_iter().rev().collect();
        let mut result = Vec::with_capacity(lhs.len() + rhs.len());
        let mut left_location = HitLocation::Outside;
        let mut right_location = HitLocation::Outside;

        loop {
            let l = lhs.pop();
            let r = rhs.pop();
            let (t, side) = match (l, r) {
                (None, None) => break,
                (Some(t), None) => (t, SideHit::Left),
                (None, Some(t)) => (t, SideHit::Right),
                (Some(left_t), Some(right_t)) => {
                    if left_t.t() < right_t.t() {
                        rhs.push(right_t);
                        (left_t, SideHit::Left)
                    } else {
                        lhs.push(left_t);
                        (right_t, SideHit::Right)
                    }
                }
            };

            match side {
                SideHit::Left => left_location = !left_location,
                SideHit::Right => right_location = !right_location,
            }
            if intersection_allowed(csg_operation, side, left_location, right_location) {
                result.push(t);
            }
        }

        result
    }
}

#[cfg(test)]
mod filter_tests {
    use crate::csg::{CSGOperation, Filter, TProvider};
    use std::fmt::{Debug, Formatter};

    #[derive(PartialEq)]
    enum LR {
        Left,
        Right,
    }

    impl Debug for LR {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                LR::Left => write!(f, "l"),
                LR::Right => write!(f, "r"),
            }
        }
    }

    #[derive(PartialEq)]
    struct LRT(LR, f32);

    impl Debug for LRT {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}!({})", &self.0, self.1)
        }
    }

    impl TProvider for LRT {
        fn t(&self) -> f32 {
            self.1
        }
    }

    macro_rules! l {
        ($f32:expr) => {
            LRT(LR::Left, $f32 as f32)
        };
    }

    macro_rules! r {
        ($f32:expr) => {
            LRT(LR::Right, $f32 as f32)
        };
    }

    macro_rules! filter_test {
        (
            lhs: $lhs:expr, rhs: $rhs:expr;
            $($name:ident; $operation:ident => $expect:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let lhs: Vec<LRT> = $lhs.into_iter().map(|t| l!(t)).collect();
                    let rhs: Vec<LRT> = $rhs.into_iter().map(|t| r!(t)).collect();
                    let filtered = Filter::filter(CSGOperation::$operation, lhs, rhs);
                    let expect: Vec<LRT> = $expect.into();
                    assert_eq!(filtered, expect);
                }
            )*
        };
    }

    // Overlaps
    filter_test!(
        lhs: [1.0, 3.0], rhs: [1.5, 3.5];
        overlapping_union;        Union        => [l!(1.0), r!(3.5)]
        overlapping_intersection; Intersection => [r!(1.5), l!(3.0)]
        overlapping_difference;   Difference   => [l!(1.0), r!(1.5)]
    );

    // Non-overlapping
    filter_test!(
        lhs: [1.0, 2.0], rhs: [2.5, 3.5];
        non_overlapping_union;        Union        => [l!(1.0), l!(2.0), r!(2.5), r!(3.5)]
        non_overlapping_intersection; Intersection => []
        non_overlapping_difference;   Difference   => [l!(1.0), l!(2.0)]
    );

    // Overlaps and non-overlaps
    filter_test!(
        lhs: [1, 2, 3, 5, 9, 10], rhs: [4, 6, 7, 8];
        multipart_union;        Union        => [l!(1), l!(2), l!(3), r!(6), r!(7), r!(8), l!(9), l!(10)]
        multipart_intersection; Intersection => [r!(4), l!(5)]
        multipart_difference;   Difference   => [l!(1), l!(2), l!(3), r!(4), l!(9), l!(10)]
    );

    // Overlaps and non-overlaps - rhs longer than lhs
    filter_test!(
        lhs: [1, 2, 3, 5, 9, 11], rhs: [4, 6, 7, 8, 10, 12, 13, 14];
        multipart_union_2;        Union        => [l!(1), l!(2), l!(3), r!(6), r!(7), r!(8), l!(9), r!(12), r!(13), r!(14)]
        multipart_intersection_2; Intersection => [r!(4), l!(5), r!(10), l!(11)]
        multipart_difference_2;   Difference   => [l!(1), l!(2), l!(3), r!(4), l!(9), r!(10)]
    );
}
