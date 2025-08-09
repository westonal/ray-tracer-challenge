#[macro_export]
macro_rules! material {
    (
        $(base: $base:expr;)?
        $(color: $color:expr;)?
        $(diffuse: $diffuse:expr;)?
        $(ambient: $ambient:expr;)?
        $(specular: $specular:expr;)?
        $(shininess: $shininess:expr;)?
        $(transparency: $transparency:expr;)?
        $(reflectivity: $reflectivity:expr;)?
        $(refractive-index: $refractive_index:expr;)?
    ) => {
        {
            let mut _material = $crate::material::Material::default();
            $(_material = $base.clone();)?
            $(_material.pattern = ray_tracer::material::pattern::Pattern::Solid($color.into());)?
            $(_material.diffuse = $diffuse as f32;)?
            $(_material.ambient = $ambient as f32;)?
            $(_material.specular = $specular as f32;)?
            $(_material.shininess = $shininess as f32;)?
            $(_material.transparency = $transparency as f32;)?
            $(_material.reflectivity = $reflectivity as f32;)?
            $(_material.refractive_index = $refractive_index as f32;)?
            _material
        }
    };
}
