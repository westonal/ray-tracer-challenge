#[cfg(test)]
mod auto_bounding_volume_tests {

    use crate::primatives::Surface;
    use crate::scene_tree::{Chain, FlatScene, FlattenScene};
    use crate::{auto, cube, scene, sphere};
    use math::matrix::matrix_4x4::Matrix4x4;
    use math::matrix4x4;

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
                matrix: matrix4x4!(translation(1., 0., 0.))
            );
        )
        .flatten_scene();
        assert_eq!(
            matrix4x4!(translation(1., 0., 0.)),
            extract_bounding_volume_matrix(&scene)
        );
    }

    #[test]
    fn bv_mirrors_transform_of_child_and_outer() {
        let scene = scene!(
            matrix: matrix4x4!(scale_all(2.));
            bounding_volume: auto!();
            +sphere!(
                matrix: matrix4x4!(translation(1., 0., 0.))
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

    fn extract_bounding_volume_matrix(flat_scene: &FlatScene) -> Matrix4x4 {
        let x = flat_scene.get(0).unwrap();
        match x {
            Chain::BoundingVolume(shape, _) => {
                assert_eq!(Surface::UnitCube, shape.surface);
                shape.transform.world_to_object_matrix()
            }
            _ => {
                panic!()
            }
        }
    }

    //TODO
    //#[test]
    fn bv_extends_to_contain_both_childs() {
        let scene = scene!(
            bounding_volume: auto!();
            +sphere!(
                matrix: matrix4x4!(translation(1., 0., 0.))
            );
            +sphere!(
                matrix: matrix4x4!(translation(-1., 0., 0.))
            );
        )
        .flatten_scene();
        assert_eq!(
            matrix4x4!(
                [0,0,0,0]
                [0,0,0,0]
                [0,0,0,0]
                [0,0,0,0]
            ),
            extract_bounding_volume_matrix(&scene)
        );
    }
}
