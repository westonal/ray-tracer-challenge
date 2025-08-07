mod sum;

use std::time::Duration;

#[derive(Debug, PartialEq)]
#[non_exhaustive] // Force use of factory macro/new
pub struct AnimationSpec {
    pub duration_limit: Duration,
    pub fps: f32,
}

impl AnimationSpec {
    pub fn new(duration: Duration, fps: f32) -> Self {
        Self {
            duration_limit: duration,
            fps,
        }
    }

    fn micros_per_frame(&self) -> u64 {
        (1000000.0 / self.fps) as u64
    }

    pub fn frame_count(&self) -> u32 {
        let frames = self.duration_limit.as_micros() as u64 / self.micros_per_frame();
        frames as u32
    }

    pub fn per_frame_time_step(&self) -> Duration {
        Duration::from_micros(self.micros_per_frame())
    }

    pub fn final_frame_start_time(&self) -> Duration {
        self.per_frame_time_step() * self.frame_count()
    }
}

#[macro_export]
macro_rules! animation_spec {
    ($seconds:expr;seconds @$fps:expr;fps) => {
        $crate::AnimationSpec::new(Duration::from_secs_f32($seconds as f32), $fps as f32)
    };
}

#[cfg(test)]
mod animation_spec_tests {
    use super::*;

    #[test]
    fn construction_macro_whole_numbers() {
        let spec = animation_spec!(10;seconds @25;fps);
        assert_eq!(25.0, spec.fps);
        assert_eq!(Duration::from_secs_f32(10.0), spec.duration_limit);
    }

    #[test]
    fn construction_macro_fractional_numbers() {
        let spec = animation_spec!(1.5;seconds @29.97;fps);
        assert_eq!(29.97, spec.fps);
        assert_eq!(Duration::from_secs_f32(1.5), spec.duration_limit);
    }

    macro_rules! frame_count {
        ($($name:ident;
           $seconds:expr;secs @$fps:expr =>
           frames: $expect_frames:expr;
           micros_per_frame: $expect_micros:expr;
           final_frame_start_at: $expect_final_frame_start_at:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let spec = $crate::animation_spec!($seconds;seconds @$fps;fps);
                    assert_eq!(spec.frame_count(), $expect_frames);
                    assert_eq!(spec.per_frame_time_step(), Duration::from_micros($expect_micros));
                    assert_eq!(spec.final_frame_start_time(), Duration::from_secs_f64($expect_final_frame_start_at as f64));
                }
            )*
        };
    }

    frame_count!(
        single_frame; 1;secs @1 => frames: 1; micros_per_frame: 1000000; final_frame_start_at: 1

        whole_number_frames; 10;secs @25 => frames: 250; micros_per_frame: 40000; final_frame_start_at: 10

        fractional_frame_rate; 8;secs @29.97 => frames: 239; micros_per_frame: 33366; final_frame_start_at: 7.974474
    );
}
