#[macro_export]
macro_rules! min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `min!` on the tail `$y`
        $x.min(min!($($y),+))
    )
}

#[macro_export]
macro_rules! max {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `max!` on the tail `$y`
        $x.max(max!($($y),+))
    )
}

#[cfg(test)]
mod min_tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(7, min!(7));
    }

    #[test]
    fn two() {
        assert_eq!(3, min!(7, 3));
        assert_eq!(3, min!(3, 7));
    }

    #[test]
    fn three() {
        assert_eq!(1, min!(1, 2, 3));
        assert_eq!(1, min!(2, 3, 1));
        assert_eq!(1, min!(3, 1, 2));
    }
}

#[cfg(test)]
mod max_tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(7, max!(7));
    }

    #[test]
    fn two() {
        assert_eq!(7, max!(7, 3));
        assert_eq!(7, max!(3, 7));
    }

    #[test]
    fn three() {
        assert_eq!(3, max!(1, 2, 3));
        assert_eq!(3, max!(2, 3, 1));
        assert_eq!(3, max!(3, 1, 2));
    }
}
