pub mod Transform;
pub mod camera;
pub mod canvas;
pub mod intersection;
pub mod lighting;
pub mod material;
pub mod primatives;
pub mod rays;
pub mod view_matrix;
pub mod world;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
