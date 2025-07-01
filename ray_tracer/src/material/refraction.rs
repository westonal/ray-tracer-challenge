use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct RefractionMediumIndexes {
    /// Refractive index of medium being left
    pub n1: f32,

    /// Refractive index of medium being entered
    pub n2: f32,
}

impl RefractionMediumIndexes {
    pub fn new(n1: f32, n2: f32) -> Self {
        Self { n1, n2 }
    }
}

pub struct RefractionStack<'s, T: Eq + Hash> {
    entered: Vec<(&'s T, f32)>,
    inside: HashSet<&'s T>,
}

impl<'s, T: Eq + Hash> RefractionStack<'s, T> {
    pub fn new() -> Self {
        Self {
            entered: Default::default(),
            inside: Default::default(),
        }
    }
    pub fn push(&mut self, key: &'s T, n: f32) -> RefractionMediumIndexes {
        let n1 = self.entered.last().map(|(_, n)| n).cloned().unwrap_or(1.);
        if self.inside.remove(key) {
            // we left this object, find and remove it from our linear list
            let (i, _) = self
                .entered
                .iter()
                .enumerate()
                .find(|(_, (k, _))| k == &key)
                .unwrap();
            self.entered.remove(i);
        } else {
            // we entered this object
            self.inside.insert(key);
            self.entered.push((key, n));
        }
        let n2 = self.entered.last().map(|(_, n)| n).cloned().unwrap_or(1.);
        RefractionMediumIndexes::new(n1, n2)
    }
}

#[cfg(test)]
mod refraction_stack_tests {
    use super::*;

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut stack = RefractionStack::new();
        assert_eq!(stack.push(&"A", 1.5), RefractionMediumIndexes::new(1., 1.5));
        assert_eq!(
            stack.push(&"B", 2.0),
            RefractionMediumIndexes::new(1.5, 2.0)
        );
        assert_eq!(
            stack.push(&"C", 2.5),
            RefractionMediumIndexes::new(2.0, 2.5)
        );
        assert_eq!(
            stack.push(&"B", 2.0),
            RefractionMediumIndexes::new(2.5, 2.5)
        );
        assert_eq!(
            stack.push(&"C", 2.5),
            RefractionMediumIndexes::new(2.5, 1.5)
        );
        assert_eq!(
            stack.push(&"A", 1.5),
            RefractionMediumIndexes::new(1.5, 1.0)
        );
    }
}
