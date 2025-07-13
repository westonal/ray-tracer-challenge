use math::point;
use math::tuple::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ObjPointIndex(usize);

#[derive(Debug, PartialEq)]
pub struct ObjTriangle(ObjPointIndex, ObjPointIndex, ObjPointIndex);

#[derive(Debug, PartialEq)]
pub struct Obj {
    points: Vec<Point>,
    triangles: Vec<ObjTriangle>,
}

type ObjResult = Result<Obj, ObjError>;

impl Default for Obj {
    fn default() -> Self {
        Self {
            points: Default::default(),
            triangles: Default::default(),
        }
    }
}

macro_rules! read_triple {
    ($t:tt, $args:expr) => {{
        let args: Vec<$t> = $args.iter().map(|f| f.parse::<$t>().unwrap()).collect();
        [
            *args.get(0).unwrap(),
            *args.get(1).unwrap(),
            *args.get(2).unwrap(),
        ]
    }};
}

macro_rules! read_triple_vec {
    ($t:tt, $args:expr) => {{
        let args: Vec<$t> = $args.iter().map(|f| f.parse::<$t>().unwrap()).collect();
        vec![
            *args.get(0).unwrap(),
            *args.get(1).unwrap(),
            *args.get(2).unwrap(),
        ]
    }};
}

#[derive(Debug, PartialEq)]
pub struct ObjError {
    line_number: usize,
    line: String,
    message: &'static str,
}

impl ObjError {
    fn new(line_idx: usize, line: &str, message: &'static str) -> Self {
        Self {
            line_number: line_idx + 1,
            line: line.to_string(),
            message,
        }
    }
}

impl TryFrom<&str> for Obj {
    type Error = ObjError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let x = value
            .split("\n")
            .enumerate()
            .map(|(i, s)| (i, s.trim()))
            .filter(|(_, s)| !s.is_empty());

        let mut points: Vec<Point> = Default::default();
        let mut triangles: Vec<ObjTriangle> = Default::default();
        for (line, s) in x {
            println!("<{}>", s);
            if s.starts_with("v ") {
                let args: &Vec<&str> = &s[2..].split_ascii_whitespace().collect();
                println!("  <{:?}>", args);
                if args.len() == 3 {
                    let args = read_triple!(f32, args);
                    points.push(point!(args[0], args[1], args[2]));
                } else {
                    return Err(ObjError::new(line, s, "Point has too few fields"));
                }
            }
            if s.starts_with("f ") {
                let args: &Vec<&str> = &s[2..].split_ascii_whitespace().collect();
                println!("  <{:?}>", args);
                if args.len() == 3 {
                    let args: Result<Vec<ObjPointIndex>, ObjError> =
                        read_triple_vec!(i32, args)
                            .into_iter()
                            .map(|i| {
                                Self::relative_index_to_absolute(i, points.len())
                                    .ok_or(ObjError::new(line, s, "Face index out of bounds"))
                            })
                            .collect();
                    let args = args?;
                    let x1: ObjPointIndex = *args.get(0).unwrap();
                    let x2: ObjPointIndex = *args.get(1).unwrap();
                    let x3: ObjPointIndex = *args.get(2).unwrap();
                    triangles.push(ObjTriangle(x1, x2, x3));
                } else {
                    return Err(ObjError::new(line, s, "Face has too few fields"));
                }
            }
        }
        Ok(Obj { points, triangles })
    }
}

impl Obj {
    fn relative_index_to_absolute(relative: i32, points_length: usize) -> Option<ObjPointIndex> {
        let points_length = points_length as i32;
        let positive = if relative < 0 {
            points_length + relative + 1
        } else {
            relative
        };
        if positive == 0 || positive > points_length {
            None
        } else {
            Some(ObjPointIndex(positive as usize - 1))
        }
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

        assert_eq!(Ok(Obj::default()), input.try_into());
    }

    #[test]
    fn read_a_single_point() {
        let input = "
            v 1 2 3.5
        ";

        let obj: Obj = input.try_into().unwrap();

        assert_eq!(vec!(point!(1, 2, 3.5)), obj.points);
    }

    #[test]
    fn read_a_single_triangle_face() {
        let input = "
            v -1 1 0
            v -1 0 0
            v 1 0 0
            f 1 2 3
        ";

        let obj: Obj = input.try_into().unwrap();

        assert_eq!(
            vec!(ObjTriangle(
                ObjPointIndex(0),
                ObjPointIndex(1),
                ObjPointIndex(2)
            )),
            obj.triangles
        );
    }

    #[test]
    fn read_a_single_triangle_face_with_negative_indexes() {
        let input = "
            v -1 1 0
            v -1 0 0
            v 1 0 0
            f -1 -3 2
        ";

        let obj: Obj = input.try_into().unwrap();

        assert_eq!(
            vec!(ObjTriangle(
                ObjPointIndex(2),
                ObjPointIndex(0),
                ObjPointIndex(1)
            )),
            obj.triangles
        );
    }
}

#[cfg(test)]
mod reader_parse_failure_tests {
    use super::*;

    #[test]
    fn a_point_with_too_few_items() {
        let input = "v 1 2 3\nv 3 4\nv 5 6 7";

        let obj: ObjResult = input.try_into();
        let error = obj.err().unwrap();
        assert_eq!("Point has too few fields", error.message);
        assert_eq!("v 3 4", error.line);
        assert_eq!(2, error.line_number);
    }

    #[test]
    fn a_face_with_an_out_of_bounds_index() {
        let input = "
            v -1 1 0
            v -1 0 0
            v 1 0 0
            f 1 2 0
            f 1 2 3
        ";

        let obj: ObjResult = input.try_into();
        let error = obj.err().unwrap();
        assert_eq!("Face index out of bounds", error.message);
        assert_eq!("f 1 2 0", error.line);
        assert_eq!(5, error.line_number);
    }

    #[test]
    fn a_face_with_fewer_than_3_items() {
        let input = "
            v -1 1 0
            v -1 0 0
            v 1 0 0
            f 1 2 3
            f 1 2
        ";

        let obj: ObjResult = input.try_into();
        let error = obj.err().unwrap();
        assert_eq!("Face has too few fields", error.message);
        assert_eq!("f 1 2", error.line);
        assert_eq!(6, error.line_number);
    }

    #[test]
    fn a_face_with_an_out_of_bounds_index_larger_than_vertex_count() {
        let input = "
            v -1 1 0
            v -1 0 0
            v 1 0 0
            f 1 2 4
            f 1 2 3
        ";

        let obj: ObjResult = input.try_into();
        let error = obj.err().unwrap();
        assert_eq!("Face index out of bounds", error.message);
        assert_eq!("f 1 2 4", error.line);
        assert_eq!(5, error.line_number);
    }

    #[test]
    fn a_face_with_an_out_of_bounds_index_smaller_than_vertex_count() {
        let input = "
            v -1 1 0
            v -1 0 0
            v 1 0 0
            f 1 2 -4
            f 1 2 3
        ";

        let obj: ObjResult = input.try_into();
        let error = obj.err().unwrap();
        assert_eq!("Face index out of bounds", error.message);
        assert_eq!("f 1 2 -4", error.line);
        assert_eq!(5, error.line_number);
    }
}
