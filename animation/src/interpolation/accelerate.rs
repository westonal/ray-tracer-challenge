use crate::interpolation::Interpolator;

pub struct Accelerate;

impl Interpolator for Accelerate {
    fn interpolate(t: f32) -> f32 {
        t * t
    }
}

#[cfg(test)]
crate::interpolation_test_mod!(
    Accelerate;
    a; 0.0  => 0.0
    b; 0.25 => 0.0625
    c; 0.5  => 0.25
    d; 0.75 => 0.5625
    e; 1.0  => 1.0
);
