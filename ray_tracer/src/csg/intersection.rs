use crate::csg::CSGOperation;
use crate::csg::intersection::HitLocation::*;
use crate::csg::intersection::SideHit::*;
use std::ops::Not;

#[derive(Clone)]
pub enum SideHit {
    /// The left hand side was hit
    Left,
    /// The right hand side was hit
    Right,
}

#[derive(PartialEq, Copy, Clone)]
pub enum HitLocation {
    /// The hit occurs on the outside of the shape
    Outside,
    /// The hit occurs on the inside of the shape
    Inside,
}

impl Not for HitLocation {
    type Output = HitLocation;

    fn not(self) -> Self::Output {
        match self {
            Outside => Inside,
            Inside => Outside,
        }
    }
}

pub(crate) fn intersection_allowed(
    csg_operation: CSGOperation,
    side_hit: SideHit,
    left_hit: HitLocation,
    right_hit: HitLocation,
) -> bool {
    match csg_operation {
        CSGOperation::Union => match side_hit {
            Left => right_hit == Outside,
            Right => left_hit == Outside,
        },
        CSGOperation::Intersection => match side_hit {
            Left => right_hit == Inside,
            Right => left_hit == Inside,
        },
        CSGOperation::Difference => match side_hit {
            Left => right_hit == Outside,
            Right => left_hit == Inside,
        },
    }
}

#[cfg(test)]
mod by_operation_intersection_filtering_tests {
    use super::*;

    macro_rules! is_allowed {
        (operation:$operation:expr; $($name:ident; side:$data:expr, left:$inl:expr, right: $inr:expr => $expect:expr)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($expect, intersection_allowed($operation, $data, $inl, $inr))
                }
            )*
        };
    }

    is_allowed!(operation:CSGOperation::Union;
        union_a; side: Left,  left: Inside,  right: Inside  => false
        union_b; side: Left,  left: Inside,  right: Outside => true
        union_c; side: Left,  left: Outside, right: Inside  => false
        union_d; side: Left,  left: Outside, right: Outside => true
        union_e; side: Right, left: Inside,  right: Inside  => false
        union_f; side: Right, left: Inside,  right: Outside => false
        union_g; side: Right, left: Outside, right: Inside  => true
        union_h; side: Right, left: Outside, right: Outside => true
    );

    is_allowed!(operation:CSGOperation::Intersection;
        intersection_a; side: Left,  left: Inside,  right: Inside  => true
        intersection_b; side: Left,  left: Inside,  right: Outside => false
        intersection_c; side: Left,  left: Outside, right: Inside  => true
        intersection_d; side: Left,  left: Outside, right: Outside => false
        intersection_e; side: Right, left: Inside,  right: Inside  => true
        intersection_f; side: Right, left: Inside,  right: Outside => true
        intersection_g; side: Right, left: Outside, right: Inside  => false
        intersection_h; side: Right, left: Outside, right: Outside => false
    );

    is_allowed!(operation:CSGOperation::Difference;
        difference_a; side: Left,  left: Inside,  right: Inside  => false
        difference_b; side: Left,  left: Inside,  right: Outside => true
        difference_c; side: Left,  left: Outside, right: Inside  => false
        difference_d; side: Left,  left: Outside, right: Outside => true
        difference_e; side: Right, left: Inside,  right: Inside  => true
        difference_f; side: Right, left: Inside,  right: Outside => true
        difference_g; side: Right, left: Outside, right: Inside  => false
        difference_h; side: Right, left: Outside, right: Outside => false
    );
}

#[cfg(test)]
mod intersection_in_flat_tree_tests {

    use crate::intersection::Intersect;
    use crate::scene_tree::FlattenScene;

    macro_rules! operation_intersections {
        ($($name:ident; $operation:tt => $expect:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let ray = $crate::ray!(math::point!(-2, 0, 0), math::vector!(1, 0, 0));
                    let cn = $crate::sphere!() $operation $crate::sphere!(matrix: math::matrix4x4!(translation(0.5, 0., 0.)));
                    let flat_scene = cn.flatten_scene();
                    let intersections = flat_scene.intersect(&ray);
                    assert_eq!($expect, intersections.iter().map(|i| i.t).collect::<Vec<_>>());
                    assert!(flat_scene.fast_hit(&ray))
                }
            )*
        };
    }

    // All intersections are [1.0, 1.5, 3.0, 3.5]
    operation_intersections!(
        union;        + => vec![1., 3.5]
        difference;   - => vec![1., 1.5]
        intersection; & => vec![1.5, 3.0]
    );
}
