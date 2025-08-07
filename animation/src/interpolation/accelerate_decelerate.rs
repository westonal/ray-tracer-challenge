use crate::interpolation::Interpolator;
use std::f32::consts::PI;

pub struct AccelerateDecelerate;

impl Interpolator for AccelerateDecelerate {
    fn interpolate(t: f32) -> f32 {
        ((t + 1.) * PI).cos() / 2. + 0.5
    }
}

#[cfg(test)]
crate::interpolation_test_mod!(
    AccelerateDecelerate;
    a; 0.0  => 0.0
    b; 0.25 => 0.14644668
    c; 0.5  => 0.5
    d; 0.75 => 0.85355353
    e; 1.0  => 1.0
);
