use crate::AABB;
use crate::primatives::{IntersectableShape, Surface};
use crate::scene_tree::{Chain, FlatScene};
use math::matrix::matrix_4x4::Matrix4x4;
use math::point;

pub(crate) fn auto_bounds_matrix(chain: &Vec<Chain>) -> Matrix4x4 {
    let mut aabb = AABB::new();
    for c in chain {
        match c {
            Chain::BoundingVolume(shape, _) => {
                let inner_aabb = shape_to_aabb(shape);
                aabb += inner_aabb;
            }
            Chain::Shape { shape, .. } => {
                let inner_aabb = shape_to_aabb(shape);
                aabb += inner_aabb;
            }
            Chain::CSG(_, _, _) => {
                todo!("Auto BV not yet implemented for CSG")
            }
        }
    }
    aabb.to_bounding_range().unwrap()
}

fn shape_to_aabb(shape: &IntersectableShape) -> AABB {
    let mut aabb = AABB::new();
    let x4 = shape.transform.object_to_world_matrix();
    match shape.surface {
        // TODO this is unsophisticated for sphere and cylinder
        //  sphere should not be subject to rotation in any axis, and cylinder can ignore y rotation
        //  The way this is, the BVs are potentially larger as they treat these primatives like cubes.
        Surface::UnitSphere | Surface::UnitCube | Surface::UnitCylinder(_) => {
            aabb.push_points(&vec![
                (x4 * point!(1, 1, 1)).force_point(),
                (x4 * point!(1, 1, -1)).force_point(),
                (x4 * point!(1, -1, 1)).force_point(),
                (x4 * point!(1, -1, -1)).force_point(),
                (x4 * point!(-1, 1, 1)).force_point(),
                (x4 * point!(-1, 1, -1)).force_point(),
                (x4 * point!(-1, -1, 1)).force_point(),
                (x4 * point!(-1, -1, -1)).force_point(),
            ])
        }
        Surface::PlaneXZ => {
            panic!("Planes cannot be within auto bounding volumes")
        }
        Surface::SingleTriangle(triangle) => {
            aabb.push_points(&triangle.vertices.map(|p| (x4 * p).force_point()));
        }
    };
    aabb
}

#[cfg(test)]
mod auto_bounding_volume_tests {

    use crate::scene_tree::FlattenScene;
    use crate::scene_tree::auto_bounding_volume::extract_bounding_volume_matrix;
    use crate::{auto, cube, scene, sphere, triangle};

    use math::{matrix4x4, point, scale, translate};

    #[test]
    fn auto_of_cube_no_transforms() {
        let scene = scene!(
            bounding_volume: auto!();
            +cube!();
        )
        .flatten_scene();
        assert_eq!(matrix4x4!(), extract_bounding_volume_matrix(&scene));
    }

    #[test]
    fn auto_of_sphere_no_transforms() {
        let scene = scene!(
            bounding_volume: auto!();
            +sphere!();
        )
        .flatten_scene();
        assert_eq!(matrix4x4!(), extract_bounding_volume_matrix(&scene));
    }

    #[test]
    fn auto_of_empty() {
        let scene = scene!(
            bounding_volume: auto!();
        )
        .flatten_scene();
        assert!(scene.is_empty());
    }

    #[test]
    fn bv_mirrors_transform_of_child() {
        let scene = scene!(
            bounding_volume: auto!();
            +sphere!(
                matrix: translate!(x:1;)
            );
        )
        .flatten_scene();
        assert_eq!(translate!(x:1;), extract_bounding_volume_matrix(&scene));
    }

    #[test]
    fn bv_mirrors_transform_of_child_and_outer() {
        let scene = scene!(
            matrix: scale!(2);
            bounding_volume: auto!();
            +sphere!(
                matrix: translate!(x: 1;)
            );
        )
        .flatten_scene();
        assert_eq!(
            matrix4x4!(
                scale_all(2.)
                translation(1., 0., 0.)
            ),
            extract_bounding_volume_matrix(&scene)
        );
    }

    #[test]
    fn bv_extends_to_contain_both_childs() {
        let scene = scene!(
            bounding_volume: auto!();
            +sphere!(
                matrix: translate!(x: 1;)
            );
            +sphere!(
                matrix: translate!(x: -1;)
            );
        )
        .flatten_scene();
        assert_eq!(
            matrix4x4!(
                [2, 0, 0, 0]
                [0, 1, 0, 0]
                [0, 0, 1, 0]
                [0, 0, 0, 1]
            ),
            extract_bounding_volume_matrix(&scene)
        );
    }

    #[test]
    fn auto_of_triangle_in_3_dimensions() {
        let scene = scene!(
            bounding_volume: auto!();
            +triangle!([point!(1, 0, 0), point!(0, 1, 0), point!(0, 0, 1)]);
        )
        .flatten_scene();
        assert_eq!(
            matrix4x4!(
                translation(0.5, 0.5, 0.5)
                scale_all(0.5)
            ),
            extract_bounding_volume_matrix(&scene)
        );
    }

    #[test]
    fn auto_of_triangle_in_2_dimensions() {
        let scene = scene!(
            bounding_volume: auto!();
            +triangle!([point!(1, 0, 0), point!(0, 1, 0), point!(1, 1, 0)]);
        )
        .flatten_scene();
        assert_eq!(
            matrix4x4!(
                translation(0.5, 0.5, 0.)
                scale_all(0.5)
                scale(1., 1., 0.1,)
            ),
            extract_bounding_volume_matrix(&scene)
        );
    }

    #[test]
    fn auto_of_triangle_in_1_dimensions() {
        let scene = scene!(
            bounding_volume: auto!();
            +triangle!([point!(0, 0, 0), point!(1, 0, 0), point!(2, 0, 0)]);
        )
        .flatten_scene();
        assert_eq!(
            matrix4x4!(
                translation(1., 0., 0.)
                scale(1., 0.05, 0.05,)
            ),
            extract_bounding_volume_matrix(&scene)
        );
    }

    #[test]
    fn auto_of_triangle_in_0_dimensions() {
        let scene = scene!(
            bounding_volume: auto!();
            +triangle!([point!(0, 0, 0), point!(0, 0, 0), point!(0, 0, 0)]);
        )
        .flatten_scene();
        assert_eq!(
            matrix4x4!(scale_all(0.05,)),
            extract_bounding_volume_matrix(&scene)
        );
    }

    #[test]
    fn auto_of_triangle_in_3_dimensions_with_transform() {
        let scene = scene!(
            matrix: translate!(x: 1;);
            bounding_volume: auto!();
            +triangle!([point!(1, 0, 0), point!(0, 1, 0), point!(0, 0, 1)]);
        )
        .flatten_scene();
        assert_eq!(
            matrix4x4!(
                translation(1.5, 0.5, 0.5)
                scale_all(0.5)
            ),
            extract_bounding_volume_matrix(&scene)
        );
    }

    #[test]
    fn nested_auto_bv() {
        let scene = scene!(
            bounding_volume: auto!();
            +sphere!(
                matrix: translate!(x: 1;)
            );
            +scene!(
                bounding_volume: auto!();
                +sphere!(
                    matrix: translate!(x: -1;)
                );
            );
        )
        .flatten_scene();
        assert_eq!(
            matrix4x4!(
                [2, 0, 0, 0]
                [0, 1, 0, 0]
                [0, 0, 1, 0]
                [0, 0, 0, 1]
            ),
            extract_bounding_volume_matrix(&scene)
        );
    }
}

#[cfg(test)]
fn extract_bounding_volume_matrix(flat_scene: &FlatScene) -> Matrix4x4 {
    let x = flat_scene.get(0).unwrap();
    match x {
        Chain::BoundingVolume(shape, _) => {
            assert_eq!(Surface::UnitCube, shape.surface);
            shape.transform.object_to_world_matrix()
        }
        _ => {
            panic!()
        }
    }
}
