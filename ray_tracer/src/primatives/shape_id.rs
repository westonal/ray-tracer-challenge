use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ShapeId {
    id: String,
}

#[cfg(test)]
impl ShapeId {
    pub(crate) fn from_string(str: &str) -> Self {
        Self {
            id: str.to_string(),
        }
    }
}

impl Display for ShapeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID:{}", self.id)
    }
}

impl Default for ShapeId {
    fn default() -> Self {
        Self {
            id: format!("{}", Uuid::new_v4()),
        }
    }
}

#[cfg(test)]
mod shape_id_tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn display() {
        assert_eq!("ID:1234", format!("{}", ShapeId::from_string("1234")));
    }

    #[test]
    fn equality() {
        assert_eq!(ShapeId::from_string("1234"), ShapeId::from_string("1234"));
    }

    #[test]
    fn inequality() {
        assert_ne!(ShapeId::from_string("1234"), ShapeId::from_string("5678"));
    }

    #[test]
    fn default_is_a_uuid_id() {
        let string = ShapeId::default().to_string();
        assert!(string.starts_with("ID:"));
        let after_id = &string[3..];
        assert!(Uuid::parse_str(after_id).is_ok());
    }

    #[test]
    fn inequality_of_random_default_ids() {
        assert_ne!(ShapeId::default(), ShapeId::default());
    }

    #[test]
    fn can_hash() {
        let mut map = HashSet::new();
        let id = ShapeId::default();
        let clone = id.clone();
        map.insert(id);
        assert!(map.contains(&clone));
        assert!(!map.contains(&ShapeId::default()));
    }
}
