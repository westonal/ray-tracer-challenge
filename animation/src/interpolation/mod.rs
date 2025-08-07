mod accelerate;
mod decelerate;
mod linear;

mod accelerate_decelerate;
#[cfg(test)]
pub mod test_macro;

pub use accelerate::Accelerate;
pub use accelerate_decelerate::AccelerateDecelerate;
pub use decelerate::Decelerate;
pub use linear::Linear;

pub trait Interpolator {
    fn interpolate(t: f32) -> f32;
}
