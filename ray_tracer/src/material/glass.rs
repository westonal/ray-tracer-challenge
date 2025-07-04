use crate::material::Material;
use crate::material::pattern::Pattern;
use math::color;

impl Material {
    pub fn glass() -> Self {
        let mut glass = Self::default();
        glass.pattern = Pattern::Solid(color!(0.1, 0.1, 0.1));
        glass.transparency = 1.;
        glass.refractive_index = 1.5;
        glass.ambient = 0.05;
        glass.diffuse = 0.1;
        glass.reflectivity = 0.5;
        glass.specular = 1.;
        glass.shininess = 300.;
        glass
    }
}

#[cfg(test)]
mod glass_tests {
    use super::*;

    #[test]
    fn glass_properties() {
        let glass = Material::glass();
        assert_eq!(1., glass.transparency);
        assert_eq!(1.5, glass.refractive_index);
    }
}

#[cfg(test)]
mod glass_refractive_index_finding_tests {
    use super::*;
    use crate::intersection::{Intersection, Intersections};

    use crate::primatives::Shape;

    use math::matrix::matrix_4x4::Matrix4x4;

    fn glass_sphere(refractive_index: f32, transform: Matrix4x4) -> Shape {
        let mut sphere = Shape::new_sphere_transformed(transform);
        sphere.material = Material::glass();
        sphere.material.refractive_index = refractive_index;
        sphere
    }

    fn run_test(index: usize) -> (f32, f32) {
        let sphere_a = glass_sphere(1.5, Matrix4x4::scale_all(2.));
        let sphere_b = glass_sphere(2.0, Matrix4x4::translation(0., 0., -0.25));
        let sphere_c = glass_sphere(2.5, Matrix4x4::translation(0., 0., 0.25));
        let index = index as f32;
        let intersections = Intersections::new(vec![
            Intersection::new(0. - index, &sphere_a),
            Intersection::new(1. - index, &sphere_b),
            Intersection::new(2. - index, &sphere_c),
            Intersection::new(3. - index, &sphere_b),
            Intersection::new(4. - index, &sphere_c),
            Intersection::new(5. - index, &sphere_a),
        ]);
        let option = intersections.hit();
        assert!(option.is_some());
        match option {
            None => {
                assert!(false);
                (-1., -1.)
            }
            Some((_, indexes)) => (indexes.n1, indexes.n2),
        }
    }

    macro_rules! refractive_index_tests {
    ($($name:ident: $index:expr => $value:expr)*) => {
    $(
        #[test]
        fn $name() {
            let (expected_n1,expected_n2) = $value;
            let (actual_n1,actual_n2) = run_test($index);
            assert_eq!(expected_n1, actual_n1, "n1");
            assert_eq!(expected_n2, actual_n2, "n2");
        }
    )*
    }
        }

    refractive_index_tests! {
        intersection_0: 0=>(1.0, 1.5)
        intersection_1: 1=>(1.5, 2.0)
        intersection_2: 2=>(2.0, 2.5)
        intersection_3: 3=>(2.5, 2.5)
        intersection_4: 4=>(2.5, 1.5)
        intersection_5: 5=>(1.5, 1.0)
    }
}
