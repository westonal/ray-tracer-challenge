use super::Interpolator;

pub struct Linear;

impl Interpolator for Linear {
    fn interpolate(t: f32) -> f32 {
        t
    }
}

#[cfg(test)]
crate::interpolation_test_mod!(
    Linear;
    a; 0.0  => 0.0
    b; 0.25 => 0.25
    c; 0.5  => 0.5
    d; 0.75 => 0.75
    e; 1.0  => 1.0
);
