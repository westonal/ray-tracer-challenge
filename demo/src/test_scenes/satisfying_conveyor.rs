use crate::obj;
use crate::obj_loader::AABBBuilder;
use crate::test_scenes::{AnimationFrame, AnimationSpec, TestScene};
use animation::animation_spec;
use image::codecs::png::FilterType::Paeth;
use math::tuple::color::{BLACK, RED, YELLOW};
use math::tuple::point::Point;
use math::{color, degrees, matrix4x4, point, vector};
use ray_tracer::camera::Camera;
use ray_tracer::canvas::Size;
use ray_tracer::intersection::Intersect;
use ray_tracer::lighting::PointLight;
use ray_tracer::material::Material;
use ray_tracer::material::pattern::Pattern;
use ray_tracer::rays::Ray;
use ray_tracer::scene_tree::{FlatScene, FlattenScene, SceneTree};
use ray_tracer::transform::Transform;
use ray_tracer::view_matrix::ViewMatrix;
use ray_tracer::world::World;
use ray_tracer::{cube, cylinder, plane, ray, scene, sphere};
use std::default::Default;
use std::f32::consts::PI;
use std::ops::Deref;
use std::panic::panic_any;
use std::time::Duration;

pub struct SatisfyingConveyor;

macro_rules! animation {
    (
        $name:tt;
        $file_name:expr;
        $animation_spec:expr;
        $scene:expr;
        $camera:expr;
    ) => {
        pub struct $name;

        impl TestScene for $name {
            fn name(&self) -> &'static str {
                $file_name
            }

            fn animation_spec(&self) -> Option<AnimationSpec> {
                Some($animation_spec)
            }

            fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
                let mut world = World::default();
                //world.max_ray_generation = 3;
                world.add($scene(frame));
                world.add_light(PointLight::new(point!(-5, 20, 10), color!(1, 1, 1)));
                world
            }

            fn build_camera_for_frame(&self, size: Size, frame: &AnimationFrame) -> Camera {
                $camera(size, frame)
            }
        }
    };
}

animation!(
    SatisfyingConveyorPt2;
    "satisfying-conveyor-pt2";
    animation_spec!(1;seconds @25;fps);
    |frame|scene!();
    |size, frame|Camera::new(size, degrees!(25));
);

impl TestScene for SatisfyingConveyor {
    fn name(&self) -> &'static str {
        "satisfying-conveyor"
    }

    fn animation_spec(&self) -> Option<AnimationSpec> {
        Some(animation_spec!(1;seconds @25;fps))
    }

    fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
        let mut factory = SatisfyingPipesAnimatedFactory::new(Mode::Efficient);
        factory.floor_height = 1.0;
        factory.die_position = 0.;
        let progress = frame.loop_progress;
        factory.conveyor_position = 0.; //frame.loop_progress;
        let factory = factory;
        let inject_progress = frame.progress;
        let inject_blob_radius = 0.54;

        let world_scene = scene!(
            //material_override: Material::default();
            +plane!(
                matrix: matrix4x4!(
                    translation(0., -factory.floor_height, 0.)
                );
                pattern: Pattern::Checker(color!(0.3, 0.3, 0.3), color!(0.7, 0.7, 0.7), Transform::identity());
            );
            //+factory.pawn.clone();
            +{
                if inject_progress > 0. {
                    scene!(
                        material_override: factory.red();
                        +factory.pawn.clone() & (
                            // Middle injection point
                            sphere!(matrix: matrix4x4!(
                                translation(0., 1., 0.)
                                scale_all(inject_blob_radius * decelerate(inject_progress))
                            )) +
                            // Top injection point
                            sphere!(matrix: matrix4x4!(
                                translation(0., 2., 0.)
                                scale_all(inject_blob_radius * decelerate(inject_progress))
                            )) +
                            // Bottom injection point
                            sphere!(matrix: matrix4x4!(
                                translation(0., 0., 0.)
                                scale_all(inject_blob_radius * decelerate(inject_progress))
                            ))
                        );
                    )
                } else {
                    scene!()
                }
            };
            // Left hand scene
            +scene!(
                +factory.half_world();
                +factory.die_stamp();
            );
            // Right hand scene
            +scene!(
                matrix: matrix4x4!(
                    scale(-1., 1., 1.)
                );
                +factory.half_world();
                // do not draw left when combined
                //+factory.die_stamp();
            );
        );

        let mut world = World::default();
        //world.max_ray_generation = 3;
        world.add(scene!(
            +world_scene;
        ));
        world.add_light(PointLight::new(point!(-5, 20, 10), color!(1, 1, 1)));
        world
    }

    fn build_camera_for_frame(&self, size: Size, frame: &AnimationFrame) -> Camera {
        let mut camera = Camera::new(size, degrees!(25));
        let top_down = ViewMatrix::new_look_at(point!(0, 10, 0), point!(0, 0, 0), vector!(0, 0, 1));
        let top_down_high =
            ViewMatrix::new_look_at(point!(0, 30, 0), point!(0, 0, 0), vector!(0, 0, 1));
        let front_on =
            ViewMatrix::new_look_at(point!(-10, 10, 20), point!(0, 0.5, 0), vector!(0, 1, 0));

        let zoom = ViewMatrix::new_look_at(
            point!(
                (accelerate_decelerate(frame.progress) * -20. + 10.) * 0.8,
                8. * 0.8,
                20. * 0.8
            ),
            point!(0, 0.5, 0),
            vector!(0, 1, 0),
        );
        camera.set_transform(zoom.into());
        camera
    }
}

//

#[derive(PartialEq)]
enum Mode {
    Efficient,
    Final,
}

struct SatisfyingPipesAnimatedFactory {
    mode: Mode,
    pipe_length: f32,
    floor_height: f32,
    pawn: SceneTree,
    die_position: f32,
    conveyor_position: f32,
    conveyor_length: f32,
    conveyor_width: f32,
    side_width: f32,
}

impl SatisfyingPipesAnimatedFactory {
    pub(crate) fn red(&self) -> Material {
        let mut material = Material::default();
        material.pattern = Pattern::Solid(*RED);
        material
    }
}

impl SatisfyingPipesAnimatedFactory {
    fn half_world(&self) -> SceneTree {
        scene!(
            matrix: matrix4x4!(
                //scale(1., 0.5, 2.)
                translation(1., -0.5, 0.)
            );
            // bounding_volume: cube!(
            //
            // );
            // conveyor
            +self.conveyor();
            +self.side();
        )
    }

    fn die_stamp(&self) -> SceneTree {
        if self.die_position > 0. {
            scene!(
                matrix: matrix4x4!(
                    translation(self.die_position, 0., 0.)
                    translation(1., 0., 0.)
                );
                material_override: Material::glass();
                +cube!(
                    matrix: matrix4x4!(
                        translation(-1., -1., 0.)
                        scale(0.5, 1.05, 1.)
                        translation(1., 1., 0.)
                        translation(0., 1., 0.)
                    );
                ) - scene!(
                        matrix: matrix4x4!(translation(-1., 0., 0.));
                        +self.pawn.clone();
                );
            )
        } else {
            // FULL block
            scene!(
                matrix: matrix4x4!(
                    translation(self.die_position, 0., 0.)
                    translation(1., 0., 0.)
                );
                material_override: Material::glass();
                +cube!(
                    matrix: matrix4x4!(
                        scale(2., 1., 1.) // scale for full block
                        translation(-1., -1., 0.)
                        scale(0.5, 1.05, 1.)
                        translation(1., 1., 0.)
                        translation(0., 1., 0.)
                    );
                ) - scene!(
                        matrix: matrix4x4!(translation(-1., 0., 0.));
                        +self.pawn.clone();
                );
            )
        }
    }

    fn conveyor(&self) -> SceneTree {
        let conveyor_length = self.conveyor_length;
        let conveyor_width = self.conveyor_width;
        cube!(
            matrix: matrix4x4!(
                translation(-1., 0., 0.)
                scale(conveyor_width / 2., 1., 1.)
                translation(1., 0., 0.)

                scale(1., 0.5, conveyor_length / 2.)
            );
            pattern: Pattern::Stripe(*YELLOW, *BLACK, Transform::new(
                matrix4x4!(
                    // To maintain the pattern's angle
                    scale(1. / conveyor_width, 1., 1. / conveyor_length)
                    rotation_y(degrees!(15))
                    rotation_y(degrees!(-90))
                    translation(self.conveyor_position, 0., 0.)
                    scale_all(0.5)
                )
            ))
        )
        .into()
    }

    fn side(&self) -> SceneTree {
        cube!(
            matrix: matrix4x4!(
                translation(0.05, 0., 0.)
                translation(self.conveyor_width, 0., 0.)

                translation(-1., 0., 0.)
                scale(self.side_width / 2., 0.5, self.conveyor_length / 2.)
                translation(1., 0., 0.)
            );
            pattern: Pattern::Solid(color!(0.7, 0.7, 0.7))
        )
        .into()
    }
}

impl SatisfyingPipesAnimatedFactory {
    fn new(mode: Mode) -> Self {
        // pawn, but can insert any obj here, it will measure and fit height to 2
        let pawn = obj_scaled_to_height_2("objs/chess/queen.obj");

        Self {
            mode,
            pipe_length: 1.,
            floor_height: 1.,
            die_position: 0.,
            conveyor_position: 0.,
            conveyor_length: 40.,
            conveyor_width: 1.,
            side_width: 1.,
            pawn,
        }
    }
}

fn obj_scaled_to_height_2(path: &str) -> SceneTree {
    let obj = obj!(path: path;);
    let measure_scene: Option<SceneTree> = match &obj {
        SceneTree::Group { bounding_shape, .. } => bounding_shape.to_owned().map(|f| f.into()),
        _ => None,
    };
    let (min, max) = extents(&measure_scene.unwrap());
    scene!(
        matrix: matrix4x4!(
            scale_all(2. / (max.y - min.y))
            // Center on x and z
            translation(-(max.x - min.x) / 2.0, 0., -(max.z - min.z) / 2.0)
            // Set origin to min point
            translation(-min.x, -min.y, -min.z)
        );
        +obj;
    )
}

fn extents(scene: &SceneTree) -> (Point, Point) {
    let scene = scene.flatten_scene();
    let mut aabb = AABBBuilder::new();
    for ray in vec![
        ray!(point!(0, 100, 0), vector!(0, -1, 0)),
        ray!(point!(0, -100, 0), vector!(0, 1, 0)),
        ray!(point!(100, 0, 0), vector!(-1, 0, 0)),
        ray!(point!(-100, 0, 0), vector!(1, 0, 0)),
        ray!(point!(0, 0, 100), vector!(0, 0, -1)),
        ray!(point!(0, 0, -100), vector!(0, 0, 1)),
    ] {
        aabb.push_point(&intersect(&scene, &ray));
    }
    (aabb.min_point(), aabb.max_point())
}

fn intersect(scene: &FlatScene, ray: &Ray) -> Point {
    let intersections = scene.intersect(&ray);
    let option = intersections.hit();
    if option.is_none() {
        panic!("No intersection, {}, {:?}", ray, intersections.len())
    }
    let (intersection, _) = option.unwrap();
    let point = ray.position(intersection.t);
    point
}

fn accelerate(input: f32) -> f32 {
    input * input
}

fn decelerate(input: f32) -> f32 {
    1.0 - accelerate(1.0 - input)
}

fn accelerate_decelerate(input: f32) -> f32 {
    ((input + 1.) * PI).cos() / 2. + 0.5
}
