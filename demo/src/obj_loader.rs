use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::point::Point;
use math::{matrix4x4, point};
use obj::{Group, Obj};
use ray_tracer::material::Material;
use ray_tracer::primatives::{Shape, Triangle};
use ray_tracer::scene_tree::SceneTree;
use ray_tracer::{AABB, cube, scene};
use std::ops::AddAssign;

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
        let mut complete_aabb = AABBBuilder::new();
        let mut complete_aabb2 = AABB::new();

        let (object, aabb, aabb2) = self.add_group(&obj, &obj.default_group);
        if object.is_not_empty() {
            scene.add(object);
            complete_aabb += aabb;
            complete_aabb2 += aabb2;
        }
        count += 1;

        for g in obj.group_names() {
            let g = &obj[g];
            let (object, aabb, aabb2) = self.add_group(&obj, g);
            scene.add(object);
            complete_aabb += aabb;
            complete_aabb2 += aabb2;
            count += 1;
        }

        let m1 = complete_aabb.to_bounding_range();
        let m2 = complete_aabb2.to_bounding_range();
        if m1 != m2 {
            panic!("{:?} != {:?}", m1, m2)
        }

        Self::scene_bounded_by(scene, &complete_aabb)
    }

    fn scene_bounded_by(scene: SceneTree, aabb: &AABBBuilder) -> SceneTree {
        if let Some(bv) = aabb.to_bounding_range().map(|m| cube!(matrix: m)) {
            scene!(
                bounding_volume: bv;
                +scene;
            )
        } else {
            scene
        }
    }

    fn add_group(&self, obj: &Obj, g: &Group) -> (SceneTree, AABBBuilder, AABB) {
        println!(
            "{}: {} Triangles",
            g.name.clone().unwrap_or("Default Group".to_string()),
            g.len()
        );

        let mut complete_aabb = AABBBuilder::new();
        let mut complete_aabb2 = AABB::new();

        let mut group = scene!();

        if g.len() == 0 {
            return (group, complete_aabb, complete_aabb2);
        }

        let mut part = scene!();
        let mut part_size = 0;

        let mut aabb = AABBBuilder::new();
        let mut aabb2 = AABB::new();
        for t in g.iter() {
            let points = obj.points.of(t);
            aabb.push_points(&points);
            aabb2.push_points(&points);

            let mut triangle = match obj.normals.of(t) {
                None => Shape::new_triangle(Triangle::new(points)),
                Some(normals) => Shape::new_triangle(Triangle::new_smooth(points, normals)),
            };

            if let Some(m) = &self.material {
                triangle.material = m.clone();
            }

            part.add(triangle);
            part_size += 1;

            if aabb.3 > 300 {
                group.add(Self::scene_bounded_by(part, &aabb));
                complete_aabb += aabb;
                complete_aabb2 += aabb2;
                part = scene!();
                part_size = 0;
                aabb = AABBBuilder::new();
                aabb2 = AABB::new();
            }
        }

        group.add(Self::scene_bounded_by(part, &aabb));
        complete_aabb += aabb;
        complete_aabb2 += aabb2;

        let m1 = complete_aabb.to_bounding_range();
        let m2 = complete_aabb2.to_bounding_range();
        if m1 != m2 {
            panic!("{:?} != {:?}", m1, m2)
        }

        (group, complete_aabb, aabb2)
    }
}

#[derive(Debug)]
struct AABBBuilderRange {
    min: f32,
    max: f32,
}

impl AABBBuilderRange {
    pub(crate) fn is_empty(&self) -> bool {
        self.min >= self.max
    }
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
    pub fn min_point(&self) -> Point {
        point!(self.0.min, self.1.min, self.2.min)
    }

    pub fn max_point(&self) -> Point {
        point!(self.0.max, self.1.max, self.2.max)
    }
}

impl AABBBuilder {
    pub(crate) fn to_bounding_range(&self) -> Option<Matrix4x4> {
        if self.0.is_empty() {
            return None;
        }
        if self.1.is_empty() {
            return None;
        }
        if self.2.is_empty() {
            return None;
        }
        Some(matrix4x4!(
            translation(self.0.min, self.1.min, self.2.min)
            scale(self.0.width(), self.1.width(), self.2.width())
            translation(0.5, 0.5, 0.5)
            scale_all(0.5)
        ))
    }
}

impl AABBBuilder {
    pub fn new() -> Self {
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
    pub fn push_point(&mut self, point: &Point) {
        self.0.push(point.x);
        self.1.push(point.y);
        self.2.push(point.z);
        self.3 += 1;
    }

    pub fn push_points(&mut self, points: &[Point]) {
        for p in points {
            self.push_point(p);
        }
    }
}

impl AddAssign for AABBBuilder {
    fn add_assign(&mut self, rhs: Self) {
        self.push_point(&rhs.min_point());
        self.push_point(&rhs.max_point());
    }
}
