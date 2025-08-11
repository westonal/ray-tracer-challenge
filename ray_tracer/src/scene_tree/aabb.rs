use crate::aabb::AABBPushable;
use crate::scene_tree::Chain;
use math::tuple::point::Point;

impl AABBPushable for Chain {
    fn extreme_points(&self) -> Vec<Point> {
        todo!()
    }
}
