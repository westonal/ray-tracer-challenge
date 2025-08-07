use crate::interpolation::{Accelerate, Interpolator};

pub struct Decelerate;

impl Interpolator for Decelerate {
    fn interpolate(t: f32) -> f32 {
        1.0 - Accelerate::interpolate(1.0 - t)
    }
}

#[cfg(test)]
crate::interpolation_test_mod!(
    Decelerate;
    a; 0.0  => 0.0
    b; 0.25 => 0.4375
    c; 0.5  => 0.75
    d; 0.75 => 0.9375
    e; 1.0  => 1.0
);
