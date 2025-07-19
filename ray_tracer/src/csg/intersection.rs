use crate::csg::CSGOperation;
use crate::csg::intersection::HitLocation::*;
use crate::csg::intersection::SideHit::*;

pub enum SideHit {
    /// The left hand side was hit
    Left,
    /// The right hand side was hit
    Right,
}

#[derive(PartialEq)]
pub enum HitLocation {
    /// The hit occurs on the outside of the shape
    Outside,
    /// The hit occurs on the inside of the shape
    Inside,
}

fn intersection_allowed(
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
mod union_intersection_filtering_tests {
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
