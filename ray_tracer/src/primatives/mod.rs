mod factories;
mod intersectable_shape;
mod intersections;
mod shape;
mod shape_id;
mod surface;
mod triangle;

pub use intersectable_shape::IntersectableShape;
pub use intersectable_shape::PointUv;
pub use intersections::CylinderCapStyle;
pub use shape::Shape;
pub use surface::Surface;
pub use shape_id::ShapeId;
pub use triangle::Triangle;
