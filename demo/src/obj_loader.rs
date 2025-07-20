use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::point::Point;
use obj::{Group, Obj};
use ray_tracer::material::Material;
use ray_tracer::primatives::{Shape, Triangle};
use ray_tracer::scene_tree::SceneTree;
use ray_tracer::{cube, scene};

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

        scene
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

        let mut aabb = AABBBuilder::new();
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

            if aabb.3 > 300 {
                group.add(scene!(
                    bounding_volume: aabb.to_bounding();
                    +part;
                ));
                part = scene!();
                aabb = AABBBuilder::new();
            }
        }

        group.add(scene!(
            bounding_volume: aabb.to_bounding();
            +part;
        ));

        group
    }
}

#[derive(Debug)]
struct AABBBuilderRange {
    min: f32,
    max: f32,
}

impl Default for AABBBuilderRange {
    fn default() -> Self {
        Self {
            min: f32::MAX,
            max: f32::MIN,
        }
    }
}

impl AABBBuilderRange {
    fn width(&self) -> f32 {
        self.max - self.min
    }
}

#[derive(Debug)]
pub struct AABBBuilder(AABBBuilderRange, AABBBuilderRange, AABBBuilderRange, usize);

impl AABBBuilder {
    pub(crate) fn to_bounding(&self) -> Shape {
        cube!(matrix: Matrix4x4::identity()
            .pre_translation(self.0.min, self.1.min, self.2.min)
            .pre_scale(self.0.width(), self.1.width(), self.2.width())
            .pre_translation(0.5, 0.5, 0.5)
            .pre_scale_all(0.5))
    }
}

impl AABBBuilder {
    fn new() -> Self {
        Self(
            Default::default(),
            Default::default(),
            Default::default(),
            0,
        )
    }
}

impl AABBBuilderRange {
    fn push(&mut self, value: f32) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
    }
}

impl AABBBuilder {
    fn push_point(&mut self, point: &Point) {
        self.0.push(point.x);
        self.1.push(point.y);
        self.2.push(point.z);
        self.3 += 1;
    }

    fn push_points(&mut self, points: &[Point]) {
        for p in points {
            self.push_point(p);
        }
    }
}
