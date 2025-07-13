use crate::test_scenes::TestScene;
use math::matrix::matrix_4x4::Matrix4x4;
use math::tuple::color::{BLACK, WHITE};
use math::tuple::point::Point;
use math::{degrees, point, vector};
use obj::reader::Obj;
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::primatives::{Shape, Triangle};
use ray_tracer::scene;
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use std::default::Default;
use std::fs;

pub struct Teapot {}

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
        Shape::new_cube_transformed(Matrix4x4::identity()
            .pre_translation(self.0.min,self.1.min,self.2.min)
            .pre_scale(
            self.0.width(),
            self.1.width(),
            self.2.width(),
        )
            .pre_translation(0.5,0.5,0.5)
            .pre_scale_all(0.5)
        )
    }
}

impl AABBBuilder {
    fn new() -> Self {
        Self(Default::default(), Default::default(), Default::default(), 0)
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
}

impl TestScene for Teapot {
    fn name() -> &'static str {
        "utah_teapot"
    }

    fn build_world() -> World {
        let obj: Obj = fs::read_to_string("objs/teapot.obj")
            .unwrap()
            .as_str()
            .try_into()
            .expect("Unable to open");

        println!("Loaded Obj");
        println!(
            "{} Points, {} Triangles",
            obj.points.len(),
            obj.triangles.len()
        );

        let mut teapot = scene!();
        let mut teapot_part = scene!();

        let mut aabb = AABBBuilder::new();
        for t in obj.triangles.iter() {
            let p1 = obj[t.0];
            let p2 = obj[t.1];
            let p3 = obj[t.2];
            aabb.push_point(&p1);
            aabb.push_point(&p2);
            aabb.push_point(&p3);
            let triangle = Shape::new_triangle(Triangle::new(p1, p2, p3));
            teapot_part.add(scene!(+triangle;));

            if aabb.3 > 300 {
                teapot.add(scene!(
                    bounding_volume: aabb.to_bounding();
                    +teapot_part;
                ));
                teapot_part = scene!();
                aabb = AABBBuilder::new();
            }
        }

        teapot.add(scene!(
            bounding_volume: aabb.to_bounding();
            +teapot_part;
        ));

        let teapot = scene!(
            +teapot;
        );

        let world_scene = scene!(
            +{
                let mut plane = Shape::new_plane();
                plane.material.pattern = Pattern::Checker(*WHITE, *BLACK, Transform::identity());
                plane
            };
            +teapot;
        );

        let mut world = World::default();
        world.add(world_scene);
        world.add_light(PointLight::new(point!(0, 20, 10), *WHITE));
        world
    }

    fn build_camera(size: Size) -> Camera {
        let mut camera = Camera::new(size, degrees!(45));
        camera.set_transform(
            ViewMatrix::new_look_at(point!(10, 8, 5), point!(0, 0, 0), vector!(0, 1, 0)).into(),
        );
        camera
    }
}
