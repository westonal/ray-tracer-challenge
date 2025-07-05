use crate::tuple::vector::Vector;

impl Vector {
    pub fn reflect(&self, normal: Vector) -> Vector {
        self.clone() - normal * 2. * self.dot(&normal)
    }
}

#[cfg(test)]
mod vector_reflection_tests {

    use crate::{assert_vector, vector};

    #[test]
    fn reflect_a_vector_at_45_degrees() {
        let vec = vector!(1., -1., 0.);
        let normal = vector!(0., 1., 0.);
        assert_vector!(vector!(1., 1., 0.), vec.reflect(normal));
    }

    #[test]
    fn reflect_a_vector_off_a_slanted_surface() {
        let vec = vector!(0., -1., 0.);
        let normal = vector!(2.0_f32.sqrt() / 2., 2.0_f32.sqrt() / 2., 0.);
        assert_vector!(vector!(1, 0, 0), vec.reflect(normal));
    }
}
