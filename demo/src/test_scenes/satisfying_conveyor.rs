use crate::obj;
use crate::obj_loader::AABBBuilder;
use crate::test_scenes::{AnimationFrame, AnimationSpec, DynamicScene, Frames, TestScene};
use animation::animation_spec;
use math::tuple::color::{BLACK, BLUE, Color, GREEN, RED, YELLOW, WHITE};
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
use ray_tracer::{cube, plane, ray, scene, sphere};
use std::default::Default;
use std::f32::consts::PI;
use std::mem;
use std::time::Duration;

const MODE: Mode = Mode::Final;
const CAMERA_MOTION: bool = false;
const FPS: usize = 30;

macro_rules! animation {
    (
        $name:tt;
        $file_name:expr;
        $animation_spec:expr;
        $scene:expr;
        $camera:expr;
    ) => {
        struct $name;

        impl TestScene for $name {
            fn name(&self) -> &'static str {
                $file_name
            }

            fn animation_spec(&self) -> Option<AnimationSpec> {
                Some($animation_spec)
            }

            fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
                let mut world = World::default();
                world.max_ray_generation = 7;
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

macro_rules! multi_part_animation {
    (
        $name:tt;
        $file_name:expr;
        [$($sub_animation:expr,)+]
    ) => {
        pub struct $name;

        impl TestScene for $name {
            fn name(&self) -> &'static str {
                $file_name
            }

            fn animation_spec(&self) -> Option<AnimationSpec> {
                Some($name.sub_scenes().iter().map(|f| f.0.animation_spec().unwrap()).sum())
            }

            fn build_world_for_frame(&self, frame: &AnimationFrame) -> World {
                let (scene, frame) = map_frame($name.sub_scenes(), frame);
                scene.0.build_world_for_frame(&frame)
            }

            fn build_camera_for_frame(&self, size: Size, frame: &AnimationFrame) -> Camera {
                let (scene, frame) = map_frame($name.sub_scenes(), frame);
                scene.0.build_camera_for_frame(size, &frame)
            }

            fn sub_scenes(&self) -> Vec<DynamicScene> {
                vec![
                    $(DynamicScene(Box::new($sub_animation),), )+
                ]
            }
        }
    };
}

fn map_frame(
    scenes: Vec<DynamicScene>,
    input_frame: &AnimationFrame,
) -> (DynamicScene, AnimationFrame) {
    let mut frame_number = input_frame.number;
    for (sub_scene_index, sub_animation) in scenes.into_iter().enumerate() {
        let spec = sub_animation.0.animation_spec().unwrap();
        let frames_in_part = spec.frame_count();
        if frame_number <= frames_in_part {
            let mut f = spec
                .build_frames()
                .get((frame_number - 1) as usize)
                .unwrap()
                .clone();
            f.sub_scene = sub_scene_index as u32;
            return (sub_animation, f);
        }
        frame_number -= frames_in_part;
    }
    panic!("Out of bounds")
}

animation!(
    SatisfyingConveyorInject;
    "satisfying-conveyor-pt.1-inject";
    animation_spec!(1;seconds @FPS;fps);
    |frame:&AnimationFrame|{
        let mut factory = SatisfyingPipesAnimatedFactory::new(MODE, frame.clone());
        factory.set_printing_material(frame.sub_scene / 4 % 2 == 0);
        factory.injection_scene()
    };
    |size, frame:&AnimationFrame|{
        let mut camera = Camera::new(size, degrees!(25));
        let motion = if CAMERA_MOTION {accelerate_decelerate(frame.progress)} else {1.};
        let zoom = ViewMatrix::new_look_at(
            point!(
                (motion * -20. + 10.) * 0.8,
                8. * 0.8,
                20. * 0.8
            ),
            point!(0, 0.5, 0),
            vector!(0, 1, 0),
        );
        camera.set_transform(zoom.into());
        camera
    };
);

animation!(
    SatisfyingConveyorRelease;
    "satisfying-conveyor-pt.2-release";
    animation_spec!(2;seconds @FPS;fps);
    |frame:&AnimationFrame|{
        let mut factory = SatisfyingPipesAnimatedFactory::new(MODE, frame.clone());
        factory.set_printing_material(frame.sub_scene / 4 % 2 == 0);
        factory.die_position = accelerate_decelerate(frame.progress);
        factory.release_scene()
    };
    |size, frame:&AnimationFrame|{
        let mut camera = Camera::new(size, degrees!(25));
        let motion = if CAMERA_MOTION {accelerate_decelerate(1.-frame.progress)} else {1.};
        let zoom = ViewMatrix::new_look_at(
            point!(
                (motion * -20. + 10.) * 0.8,
                8. * 0.8,
                20. * 0.8
            ),
            point!(0, 0.5, 0),
            vector!(0, 1, 0),
        );
        camera.set_transform(zoom.into());
        camera
    };
);

animation!(
    SatisfyingConveyorMove;
    "satisfying-conveyor-pt.3-move";
    animation_spec!(2;seconds @FPS;fps);
    |frame:&AnimationFrame|{
        let mut factory = SatisfyingPipesAnimatedFactory::new(MODE, frame.clone());
        factory.die_position = 1.;
        factory.conveyor_position = factory.conveyor_motion_per_cycle/2. * accelerate(frame.progress);
        factory.set_printing_material(frame.sub_scene / 4 % 2 == 0);
        factory.conveyor_move_scene()
    };
    |size, frame:&AnimationFrame|{
        let mut camera = Camera::new(size, degrees!(25));
        let zoom = ViewMatrix::new_look_at(
            point!(
                -10. * 0.8,
                8. * 0.8,
                20. * 0.8
            ),
            point!(0, 0.5, 0),
            vector!(0, 1, 0),
        );
        camera.set_transform(zoom.into());
        camera
    };
);

animation!(
    SatisfyingConveyorClose;
    "satisfying-conveyor-pt.4-close";
    animation_spec!(2;seconds @FPS;fps);
    |frame:&AnimationFrame|{
        let mut factory = SatisfyingPipesAnimatedFactory::new(MODE, frame.clone());
        factory.die_position = accelerate_decelerate(1. - frame.progress);
        factory.conveyor_position = factory.conveyor_motion_per_cycle/2. * decelerate(frame.progress);
        factory.set_printing_material(frame.sub_scene / 4 % 2 == 0);
        factory.close_scene()
    };
    |size, frame:&AnimationFrame|{
        let mut camera = Camera::new(size, degrees!(25));
        let motion = if CAMERA_MOTION {accelerate_decelerate(1.-frame.progress)} else {1.};
        let zoom = ViewMatrix::new_look_at(
            point!(
                (motion * -20. + 10.) * 0.8,
                8. * 0.8,
                20. * 0.8
            ),
            point!(0, 0.5, 0),
            vector!(0, 1, 0),
        );
        camera.set_transform(zoom.into());
        camera
    };
);

multi_part_animation!(
    SatisfyingConveyor;
    "satisfying-conveyor";
    [
        SatisfyingConveyorInject,
        SatisfyingConveyorRelease,
        SatisfyingConveyorMove,
        SatisfyingConveyorClose,
        // Repeat for second color
        SatisfyingConveyorInject,
        SatisfyingConveyorRelease,
        SatisfyingConveyorMove,
        SatisfyingConveyorClose,
    ]
);

#[derive(PartialEq)]
enum Mode {
    Efficient,
    Middle,
    Final,
}

struct SatisfyingPipesAnimatedFactory {
    mode: Mode,
    frame: AnimationFrame,
    floor_height: f32,
    injection_object: SceneTree,
    die_position: f32,
    conveyor_position: f32,
    conveyor_length: f32,
    conveyor_width: f32,
    conveyor_motion_per_cycle: f32,
    prior_objects_on_belt: usize,
    side_width: f32,
    printing_material: Material,
    use_full_die: bool,
    prior_material_odd: Material,
    prior_material_even: Material,
}

impl SatisfyingPipesAnimatedFactory {
    pub(crate) fn set_printing_material(&mut self, even: bool) {
        if !even {
            mem::swap(&mut self.prior_material_odd, &mut self.prior_material_even);
        }
        self.printing_material = self.prior_material_even.clone()
    }
}

impl SatisfyingPipesAnimatedFactory {
    pub(crate) fn background(&self) -> SceneTree {
        plane!(
            matrix: matrix4x4!(
                translation(0., -self.floor_height, 0.)
            );
            pattern: Pattern::Checker(color!(0.3, 0.3, 0.3), color!(0.7, 0.7, 0.7), Transform::identity());
        ).into()
    }

    pub(crate) fn injection_scene(&self) -> SceneTree {
        let inject_progress = self.frame.progress;
        let inject_blob_radius = 0.54;

        scene!(
            +self.background();
            +self.prior_prints(1);
            +{
                if inject_progress > 0. {
                    scene!(
                        material_override: self.printing_material.clone();
                        +self.injection_object.clone() & (
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
                +self.half_world();
                +self.half_die_stamp_left();
            );
            // Right hand scene
            +scene!(
                matrix: matrix4x4!(
                    scale(-1., 1., 1.)
                );
                +self.half_world();
                +self.half_die_stamp_right();
            );
        )
    }

    pub(crate) fn release_scene(&self) -> SceneTree {
        scene!(
            +self.background();
            +self.prior_prints(1);
            +scene!(
                material_override: self.printing_material.clone();
                +self.injection_object.clone();
            );
            // Left hand scene
            +scene!(
                +self.half_world();
                +self.half_die_stamp_left();
            );
            // Right hand scene
            +scene!(
                matrix: matrix4x4!(
                    scale(-1., 1., 1.)
                );
                +self.half_world();
                +self.half_die_stamp_right();
            );
        )
    }

    pub(crate) fn close_scene(&self) -> SceneTree {
        scene!(
            +self.background();
            +self.prior_prints(0);
            // Left hand scene
            +scene!(
                +self.half_world();
                +self.half_die_stamp_left();
            );
            // Right hand scene
            +scene!(
                matrix: matrix4x4!(
                    scale(-1., 1., 1.)
                );
                +self.half_world();
                +self.half_die_stamp_right();
            );
        )
    }

    pub(crate) fn conveyor_move_scene(&self) -> SceneTree {
        scene!(
            +self.background();
            +self.prior_prints(1);
            +scene!(
                matrix: matrix4x4!(
                    translation(0.,0.,self.conveyor_position)
                );
                +scene!(
                    material_override: self.printing_material.clone();
                    +self.injection_object.clone();
                );
            );
            // Left hand scene
            +scene!(
                +self.half_world();
                +self.half_die_stamp_left();
            );
            // Right hand scene
            +scene!(
                matrix: matrix4x4!(
                    scale(-1., 1., 1.)
                );
                +self.half_world();
                +self.half_die_stamp_right();
            );
        )
    }

    /// Place prior printed objects on the belt
    fn prior_prints(&self, offset: usize) -> SceneTree {
        let mut scene = scene!();
        for i in 0..self.prior_objects_on_belt {
            let i = i + offset + 1;
            scene.add(scene!(
                matrix: matrix4x4!(
                    translation(0., 0., (self.conveyor_motion_per_cycle/2.) * i as f32)
                    translation(0.,0.,self.conveyor_position)
                );
                +scene!(
                    material_override: if i % 2 == 1 {
                        self.prior_material_even.clone()
                    } else {
                        self.prior_material_odd.clone()
                    };
                    +self.injection_object.clone();
                );
            ))
        }
        scene
    }
}

impl SatisfyingPipesAnimatedFactory {
    fn half_world(&self) -> SceneTree {
        scene!(
            matrix: matrix4x4!(
                translation(1., -0.5, 0.)
            );
            +self.conveyor();
            +self.side();
        )
    }

    fn half_die_stamp_left(&self) -> SceneTree {
        if self.use_full_die && self.die_position == 0. {
            self.full_die_stamp()
        } else {
            self.half_die_stamp()
        }
    }

    fn half_die_stamp_right(&self) -> SceneTree {
        scene!(
            iff: self.mode != Mode::Efficient && (!self.use_full_die || self.die_position > 0.);
            +self.half_die_stamp();
        )
    }

    fn half_die_stamp(&self) -> SceneTree {
        scene!(
            matrix: matrix4x4!(
                translation(self.die_position, 0., 0.)
                translation(0.01, 0., 0.)
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
                    +self.injection_object.clone();
            );
        )
    }
    
    fn full_die_stamp(&self) -> SceneTree {
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
                    +self.injection_object.clone();
            );
        )
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
                    translation(self.conveyor_position * 2., 0., 0.)
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
    fn new(mode: Mode, frame: AnimationFrame) -> Self {
        let injection_object = match mode {
            // can insert any obj here, it will measure and fit height to 2
            Mode::Final => obj_scaled_to_height_2("objs/chess/queen.obj"),
            _ => {
                sphere!(matrix: matrix4x4!(scale_all(0.5) scale(1., 2., 1.) translation(0.,1.,0.)))
                    .into()
            }
        };

        Self {
            mode,
            frame,
            floor_height: 1.,
            die_position: 0.,
            conveyor_position: 0.,
            conveyor_length: 40.,
            conveyor_width: 1.,
            conveyor_motion_per_cycle: 4.,
            prior_objects_on_belt: 1,
            side_width: 1.5,
            use_full_die: true,
            printing_material: {
                let mut mat = Material::default();
                mat.pattern = Pattern::Solid(*GREEN);
                mat
            },
            prior_material_odd: {
                let mut mat = Material::default();
                mat.pattern = Pattern::Solid(*WHITE);
                mat
            },
            prior_material_even: {
                let mut mat = Material::default();
                mat.pattern = Pattern::Solid(*BLACK);
                mat
            },
            injection_object,
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
