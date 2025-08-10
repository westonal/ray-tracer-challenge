use obj::{Group, Obj};
use ray_tracer::material::Material;
use ray_tracer::primatives::{Shape, Triangle};
use ray_tracer::scene_tree::SceneTree;
use ray_tracer::{auto, scene};

#[macro_export]
macro_rules! obj {
    (path: $path:expr; $(material: $material:expr;)?) => {{
        let obj: obj::Obj = std::fs::read_to_string($path)
            .unwrap()
            .as_str()
            .try_into()
            .expect(&format!("Unable to open {}", $path));

        println!("Loaded Obj {}", $path);
        let mut _mat = None;
        $(_mat = Some($material);)?
        $crate::obj_loader::ObjLoader::new(_mat).obj_to_scene(&obj)
    }};
}

pub struct ObjLoader {
    material: Option<Material>,
}

impl ObjLoader {
    pub fn new(material: Option<Material>) -> Self {
        Self { material }
    }

    pub fn obj_to_scene(&self, obj: &Obj) -> SceneTree {
        let mut scene = scene!();

        scene.add(self.add_group(&obj, &obj.default_group));

        for g in obj.group_names() {
            let g = &obj[g];
            scene.add(self.add_group(&obj, g));
        }

        auto_bv_scene(scene)
    }

    fn add_group(&self, obj: &Obj, g: &Group) -> SceneTree {
        println!(
            "{}: {} Triangles",
            g.name.clone().unwrap_or("Default Group".to_string()),
            g.len()
        );

        let mut group = scene!();

        if g.len() == 0 {
            return group;
        }

        let mut part = scene!();
        let mut part_size = 0;

        for t in g.iter() {
            let points = obj.points.of(t);

            let mut triangle = match obj.normals.of(t) {
                None => Shape::new_triangle(Triangle::new(points)),
                Some(normals) => Shape::new_triangle(Triangle::new_smooth(points, normals)),
            };

            if let Some(m) = &self.material {
                triangle.material = m.clone();
            }

            part.add(triangle);
            part_size += 1;

            if part_size > 100 {
                group.add(auto_bv_scene(part));
                part = scene!();
                part_size = 0;
            }
        }

        group.add(auto_bv_scene(part));

        group
    }
}

fn auto_bv_scene(scene: SceneTree) -> SceneTree {
    scene!(
        bounding_volume: auto!();
        +scene;
    )
}
