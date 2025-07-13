use math::point;
use math::tuple::point::Point;

pub struct Obj {
    points: Vec<Point>,
}

impl From<&str> for Obj {
    fn from(value: &str) -> Self {
        let x = value
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());

        let mut points: Vec<Point> = Default::default();
        for s in x {
            println!("<{}>", s);
            if s.starts_with("v ") {
                let args: &Vec<&str> = &s[2..].split_ascii_whitespace().collect();
                println!("  <{:?}>", args);
                if args.len() == 3 {
                    let args: Vec<f32> = args.iter().map(|f| f.parse::<f32>().unwrap()).collect();
                    points.push(point!(
                        *args.get(0).unwrap(),
                        *args.get(1).unwrap(),
                        *args.get(2).unwrap()
                    ));
                }
            }
        }
        Obj { points }
    }
}

#[cfg(test)]
mod reader_tests {
    use super::*;
    use math::point;

    #[test]
    fn ignore_lines_not_that_are_not_understood() {
        let input = "
            nothing
            readable
            here
        ";

        let obj: Obj = input.into();
    }

    #[test]
    fn read_a_single_point() {
        let input = "
            v 1 2 3.5
        ";

        let obj: Obj = input.into();

        assert_eq!(vec!(point!(1, 2, 3.5)), obj.points);
    }
}
