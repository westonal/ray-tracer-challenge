use obj::{Group, Obj};
use ray_tracer::material::Material;
use ray_tracer::primatives::{Shape, Triangle};
use ray_tracer::scene_tree::SceneTree;
use ray_tracer::{cube, scene, AABB};

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
        let mut count = 0;
        let mut complete_aabb = AABB::new();

        let (object, aabb) = self.add_group(&obj, &obj.default_group);
        if object.is_not_empty() {
            scene.add(object);
            complete_aabb += aabb;
        }
        count += 1;

        for g in obj.group_names() {
            let g = &obj[g];
            let (object, aabb) = self.add_group(&obj, g);
            scene.add(object);
            complete_aabb += aabb;
            count += 1;
        }

        Self::scene_bounded_by(scene, &complete_aabb)
    }

    fn scene_bounded_by(scene: SceneTree, aabb: &AABB) -> SceneTree {
        if let Some(bv) = aabb.to_bounding_range().map(|m| cube!(matrix: m)) {
            scene!(
                bounding_volume: bv;
                +scene;
            )
        } else {
            scene
        }
    }

    fn add_group(&self, obj: &Obj, g: &Group) -> (SceneTree, AABB) {
        println!(
            "{}: {} Triangles",
            g.name.clone().unwrap_or("Default Group".to_string()),
            g.len()
        );

        let mut complete_aabb = AABB::new();

        let mut group = scene!();

        if g.len() == 0 {
            return (group, complete_aabb);
        }

        let mut part = scene!();
        let mut part_size = 0;

        let mut aabb = AABB::new();
        for t in g.iter() {
            let points = obj.points.of(t);
            aabb.push_points(&points);

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
                group.add(Self::scene_bounded_by(part, &aabb));
                complete_aabb += aabb;
                part = scene!();
                part_size = 0;
                aabb = AABB::new();
            }
        }

        group.add(Self::scene_bounded_by(part, &aabb));
        complete_aabb += aabb;

        (group, complete_aabb)
    }
}
