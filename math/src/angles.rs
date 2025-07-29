use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Angle(f32);

impl Angle {
    pub fn radians(rads: f32) -> Self {
        Self(rads)
    }

    pub fn degrees(degrees: f32) -> Self {
        Self(degrees * PI / 180.)
    }

    pub fn to_radians(&self) -> f32 {
        self.0
    }

    pub fn to_degrees(&self) -> f32 {
        self.to_radians() * 180. / PI
    }

    pub fn sin_cos(&self) -> (f32, f32) {
        self.0.sin_cos()
    }
}

#[macro_export]
macro_rules! radians {
    ($e:expr) => {
        $crate::Angle::radians($e as f32)
    };
}

#[macro_export]
macro_rules! degrees {
    ($e:expr) => {
        $crate::Angle::degrees($e as f32)
    };
}

#[cfg(test)]
mod angle_tests {

    use std::f32::consts::PI;

    #[test]
    fn create_radians() {
        let a = radians!(PI);

        assert_eq!(a.to_radians(), PI);
        assert_eq!(a.to_degrees(), 180.0);
    }

    #[test]
    fn create_radians_2() {
        let a = radians!(PI / 2.);

        assert_eq!(a.to_radians(), PI / 2.);
        assert_eq!(a.to_degrees(), 90.0);
    }

    #[test]
    fn create_degrees() {
        let a = degrees!(180);

        assert_eq!(a.to_degrees(), 180.0);
        assert_eq!(a.to_radians(), PI);
    }

    #[test]
    fn create_degrees_2() {
        let a = degrees!(90.);

        assert_eq!(a.to_degrees(), 90.);
        assert_eq!(a.to_radians(), PI / 2.);
    }

    #[test]
    fn sin_cos_90() {
        let (sin, cos) = radians!(PI / 2.).sin_cos();
        assert_eq!(sin, 1.);
        assert_eq!(cos, -4.371139e-8);
    }

    #[test]
    fn sin_cos_180() {
        let (sin, cos) = degrees!(180.).sin_cos();
        assert_eq!(sin, -8.742278e-8);
        assert_eq!(cos, -1.);
    }
}
