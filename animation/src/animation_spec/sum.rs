use crate::AnimationSpec;
use std::iter::Sum;
use std::time::Duration;

impl Sum for AnimationSpec {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut fps = None;
        let mut duration = Duration::from_secs(0);
        for i in iter {
            duration += i.duration_limit;
            fps = fps.or(Some(i.fps));
        }
        AnimationSpec::new(duration, fps.expect("Can't sum zero animation specs"))
    }
}

#[cfg(test)]
mod animation_spec_sum_tests {
    use super::*;
    use crate::animation_spec;

    #[test]
    #[should_panic]
    fn sum_of_no_specs_is_not_possible() {
        let v: Vec<AnimationSpec> = vec![];
        v.into_iter().sum::<AnimationSpec>();
    }

    #[test]
    fn one_spec() {
        let v = vec![animation_spec!(1;seconds @25;fps)];
        assert_eq!(
            v.into_iter().sum::<AnimationSpec>(),
            animation_spec!(1;seconds @25;fps)
        );
    }

    #[test]
    fn two_specs() {
        let v = vec![
            animation_spec!(1;seconds @25;fps),
            animation_spec!(2;seconds @25;fps),
        ];
        assert_eq!(
            v.into_iter().sum::<AnimationSpec>(),
            animation_spec!(3;seconds @25;fps)
        );
    }

    #[test]
    fn three_specs_at_another_fps() {
        let v = vec![
            animation_spec!(1;seconds @30;fps),
            animation_spec!(2;seconds @30;fps),
            animation_spec!(3;seconds @30;fps),
        ];
        assert_eq!(
            v.into_iter().sum::<AnimationSpec>(),
            animation_spec!(6;seconds @30;fps)
        );
    }

    #[test]
    fn inconsistent_fps_takes_the_first() {
        let v = vec![
            animation_spec!(3;seconds @10;fps),
            animation_spec!(4;seconds @20;fps),
        ];
        assert_eq!(
            v.into_iter().sum::<AnimationSpec>(),
            animation_spec!(7;seconds @10;fps)
        );
    }
}
