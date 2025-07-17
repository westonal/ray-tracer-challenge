use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct FaceTriple {
    pub index: i32,
    pub texture: Option<i32>,
    pub normal: Option<i32>,
}

impl FaceTriple {
    fn new(index: i32, texture: Option<i32>, normal: Option<i32>) -> Self {
        Self {
            index,
            texture,
            normal,
        }
    }
}

impl FromStr for FaceTriple {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.splitn(3, "/").collect();

        Ok(FaceTriple::new(
            i32::from_str(split[0])?,
            parse_option(split.get(1))?,
            parse_option(split.get(2))?,
        ))
    }
}

fn parse_option(input: Option<&&str>) -> Result<Option<i32>, ParseIntError> {
    match input {
        None => Ok(None),
        Some(&"") => Ok(None),
        Some(s) => Ok(Some(i32::from_str(s)?)),
    }
}

#[cfg(test)]
mod face_triple_str_parse_tests {
    use super::*;
    use FaceTriple;

    macro_rules! assert_triples {
        ($($name:ident; $data:expr => $expect:expr)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($expect, FaceTriple::from_str($data).unwrap());
                }
            )*
        };
    }

    assert_triples!(
        just_index;               "1"     => FaceTriple::new(1, None, None)
        just_negative_index;      "-2"    => FaceTriple::new(-2, None, None)
        index_and_texture;        "2/3"   => FaceTriple::new(2, Some(3), None)
        index_texture_and_normal; "3/4/5" => FaceTriple::new(3, Some(4), Some(5))
        index_and_normal;         "3//5"  => FaceTriple::new(3, None, Some(5))
    );
}
