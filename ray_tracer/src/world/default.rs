#[cfg(test)]
pub mod test_world {
    use crate::lighting::PointLight;
    use crate::material::Material;
    use crate::material::pattern::Pattern;

    use crate::sphere;
    use crate::world::World;
    use math::matrix::matrix_4x4::*;
    use math::{color, point};

    /// A test world
    impl World {
        pub fn default_world() -> World {
            let mut world = World::default();
            world.set_light(PointLight::new(point!(-10, 10, -10), color!(1.0, 1.0, 1.0)));
            let mut sphere = sphere!();
            let mut material = Material::default();
            material.pattern = Pattern::Solid(color!(0.8, 1., 0.6));
            material.diffuse = 0.7;
            material.specular = 0.2;
            // turn off shadows
            material.shadow_boost = 1.;
            sphere.material = material;
            world.add(sphere);
            let mut material = Material::default();
            material.shadow_boost = 1.;
            let mut sphere = sphere!(matrix: Matrix4x4::scale(0.5, 0.5, 0.5));
            sphere.material = material;
            world.add(sphere);
            world
        }
    }
}
