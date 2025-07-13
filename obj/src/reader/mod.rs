mod obj_triangle;
mod point_collection;

pub use crate::reader::obj_triangle::ObjTriangle;
pub use crate::reader::point_collection::PointCollection;
use math::point;
use math::tuple::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ObjPointIndex(usize);

#[derive(Debug, PartialEq)]
pub struct Obj {
    points: PointCollection,
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

macro_rules! parse_vec {
    ($t:tt, $args:expr) => {{
        let args: Vec<$t> = $args.iter().map(|f| f.parse::<$t>().unwrap()).collect();
        args
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
            .map(|s| {
                let comment = s.find("#");
                if let Some(idx) = comment {
                    &s[0..idx]
                } else {
                    s
                }
            })
            .map(|s| s.trim())
            .enumerate()
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
                if args.len() >= 3 {
                    let args: Result<Vec<ObjPointIndex>, ObjError> =
                        parse_vec!(i32, args)
                            .into_iter()
                            .map(|i| {
                                Self::relative_index_to_absolute(i, points.len())
                                    .ok_or(ObjError::new(line, s, "Face index out of bounds"))
                            })
                            .collect();
                    let args = args?;
                    triangles.append(&mut to_triangles(args));
                } else {
                    return Err(ObjError::new(line, s, "Face has too few fields"));
                }
            }
        }
        Ok(Obj {
            points: points.into(),
            triangles,
        })
    }
}

fn to_triangles(points: Vec<ObjPointIndex>) -> Vec<ObjTriangle> {
    let mut result = vec![];
    let p1: ObjPointIndex = *points.get(0).unwrap();
    let mut p2: ObjPointIndex = *points.get(1).unwrap();
    for p in points.into_iter().skip(2) {
        result.push([p1, p2, p].into());
        p2 = p;
    }
    result
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
    use math::{assert_point, point};

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

        assert_eq!(vec!(point!(1, 2, 3.5)), *obj.points);
    }

    #[test]
    fn read_a_single_point_with_comment() {
        let input = "
            v 1 2 3.5 # 4 5 6
        ";

        let obj: Obj = input.try_into().unwrap();

        assert_eq!(vec!(point!(1, 2, 3.5)), *obj.points);
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

        let expected: ObjTriangle = [ObjPointIndex(0), ObjPointIndex(1), ObjPointIndex(2)].into();
        assert_eq!(vec!(expected), obj.triangles);
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

        let expected: ObjTriangle = [ObjPointIndex(2), ObjPointIndex(0), ObjPointIndex(1)].into();
        assert_eq!(vec!(expected), obj.triangles);
    }

    #[test]
    fn access_points_by_index() {
        let input = "
            v -1 1 1
            v -1 0 2
            v 1 0 3
            f 1 2 3
            f -1 -3 2
        ";

        let obj: Obj = input.try_into().unwrap();

        let triangle = obj.triangles.get(0).unwrap();
        assert_point!(point!(-1, 1, 1), obj.points[triangle[0]]);
        assert_point!(point!(-1, 0, 2), obj.points[triangle[1]]);
        assert_point!(point!(1, 0, 3), obj.points[triangle[2]]);

        let triangle = obj.triangles.get(1).unwrap();
        assert_point!(point!(1, 0, 3), obj.points[triangle[0]]);
        assert_point!(point!(-1, 1, 1), obj.points[triangle[1]]);
        assert_point!(point!(-1, 0, 2), obj.points[triangle[2]]);
    }

    macro_rules! assert_triangles {
        ($obj: expr; $($i:expr => $p1:expr, $p2:expr, $p3:expr;)+) => {
            {
            $(
                let triangle = $obj.triangles.get($i).unwrap();
                assert_eq!(
                    [$p1, $p2, $p3],
                    $obj.points.of(triangle)
                );
            )+
            }
        };
    }

    #[test]
    fn access_points_by_triangle() {
        let input = "
            v -1 1 1
            v -1 0 2
            v 1 0 3
            f 1 2 3
            f -1 -3 2
        ";

        let obj: Obj = input.try_into().unwrap();

        assert_triangles!(obj;
            0 => point!(-1, 1, 1), point!(-1, 0, 2), point!(1, 0, 3);
            1 => point!(1, 0, 3), point!(-1, 1, 1), point!(-1, 0, 2);
        );
    }

    #[test]
    fn triangulating_polygons() {
        let input = "
            v -1 1 11
            v -1 0 12
            v  1 0 13
            v  1 1 14
            v  0 2 15
            f 1 2 3 4 5
        ";

        let obj: Obj = input.try_into().unwrap();
        assert_eq!(3, obj.triangles.len());
        assert_triangles!(obj;
            0 => point!(-1, 1, 11), point!(-1, 0, 12), point!(1, 0, 13);
            1 => point!(-1, 1, 11), point!( 1, 0, 13), point!(1, 1, 14);
            2 => point!(-1, 1, 11), point!( 1, 1, 14), point!(0, 2, 15);
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
