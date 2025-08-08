use crate::material::Material;
use crate::scene_tree::flat_scene::{Chain, FlatScene};
use crate::scene_tree::{AUTO_CUBE_BOUNDING_VOLUME, SceneTree};
use crate::{chain_link, cube};
use math::matrix::matrix_4x4::Matrix4x4;
use math::matrix4x4;
use crate::primatives::Surface;

pub trait FlattenScene {
    fn flatten_scene(&self) -> FlatScene;
}

pub(crate) trait FlattenSceneWithMatrix {
    fn flatten_with_matrix(&self, matrix: Matrix4x4) -> FlatScene;
}

impl<T: FlattenSceneWithMatrix> FlattenScene for T {
    fn flatten_scene(&self) -> FlatScene {
        self.flatten_with_matrix(matrix4x4!())
    }
}

impl FlattenSceneWithMatrix for SceneTree {
    fn flatten_with_matrix(&self, matrix4x4: Matrix4x4) -> FlatScene {
        let mut chain = vec![];
        self.walk(&mut chain, matrix4x4, &Overrides::default());
        FlatScene::new(chain)
    }
}

#[derive(Default, Clone)]
struct Overrides {
    material_override: Option<Material>,
}

impl Overrides {
    fn clone_with(&self, material: &Material) -> Self {
        let mut clone = self.clone();
        clone.material_override = Some(material.clone());
        clone
    }
}

impl SceneTree {
    fn walk(&self, into: &mut Vec<Chain>, tree_matrix: Matrix4x4, overrides: &Overrides) {
        match self {
            SceneTree::Leaf(shape) => {
                let mut shape = (*shape).clone();
                shape.matrix = tree_matrix * shape.matrix;
                shape.material = overrides
                    .material_override
                    .to_owned()
                    .unwrap_or(shape.material);
                into.push(chain_link!(shape))
            }
            SceneTree::CsgLeaf(lhs_tree, operation, rhs_tree) => {
                let mut lhs_chain = vec![];
                lhs_tree.walk(&mut lhs_chain, tree_matrix, overrides);
                let mut rhs_chain = vec![];
                rhs_tree.walk(&mut rhs_chain, tree_matrix, overrides);
                into.push(Chain::CSG(*operation, lhs_chain.len(), rhs_chain.len()));
                into.append(&mut lhs_chain);
                into.append(&mut rhs_chain);
            }
            SceneTree::Group {
                matrix,
                material_override,
                bounding_shape,
                children,
            } => {
                let matrix = tree_matrix * *matrix;

                let overrides: &Overrides = if let Some(material) = material_override {
                    &overrides.clone_with(material)
                } else {
                    overrides
                };

                match bounding_shape {
                    None => {
                        for child in children {
                            child.walk(into, matrix, overrides);
                        }
                    }
                    Some(bounds) => {
                        let mut subtree = vec![];
                        for child in children {
                            child.walk(&mut subtree, matrix, overrides);
                        }

                        if !subtree.is_empty() {
                            let bounds = if bounds.id == AUTO_CUBE_BOUNDING_VOLUME.id {
                                let bounds_matrix = auto_bounds_matrix(&subtree);
                                cube!(matrix: bounds_matrix)
                            } else {
                                let mut bounds = bounds.clone();
                                bounds.matrix = matrix * bounds.matrix;
                                bounds
                            };
                            into.push(chain_link!(bounds, skip: subtree.len()));
                            // bounds2.material.transparency = 0.9;
                            // into.push(Chain::Shape(
                            //     bounds2.to_intersectable(),
                            //     //subtree.len(),
                            // ));
                            into.append(&mut subtree);
                        }
                    }
                }
            }
        }
    }
}

fn auto_bounds_matrix(chain: &Vec<Chain>) -> Matrix4x4 {
    let mut result = matrix4x4!();
    if chain.len() > 1{
        todo!("Multiple shapes not yet implemented")
    }
    for c in chain{
        match c {
            Chain::BoundingVolume(_, _) => {
                todo!("Auto BV not yet implemented for BVs")
            }
            Chain::Shape(s) => {
                result = s.transform.world_to_object_matrix();
                match s.surface {
                    Surface::UnitSphere => {}
                    Surface::UnitCube => {}
                    Surface::PlaneXZ => {}
                    Surface::UnitCylinder(_) => {}
                    Surface::SingleTriangle(_) => {}
                }
            }
            Chain::CSG(_, _, _) => {
                todo!("Auto BV not yet implemented for CSG")
            }
        }
    }
    result
}

#[cfg(test)]
mod flatten_tests {
    use super::*;

    use crate::{cube, plane, sphere};

    #[test]
    fn flatten_one() {
        let mut tree = SceneTree::default();
        tree.add(sphere!());

        let vec = tree.flatten_scene();
        assert_eq!(1, vec.len());
    }

    #[test]
    fn flatten_two() {
        let mut tree = SceneTree::default();
        tree.add(sphere!());
        tree.add(cube!());

        let vec = tree.flatten_scene();
        assert_eq!(2, vec.len());
    }

    #[test]
    fn flatten_two_in_sub_tree() {
        let mut tree = SceneTree::default();
        tree.add(sphere!());

        let mut branch = SceneTree::default();
        branch.add(cube!());

        tree.add(branch);

        let vec = tree.flatten_scene();
        assert_eq!(2, vec.len());
    }

    #[test]
    fn flatten_three_in_sub_tree() {
        let mut tree = SceneTree::default();
        tree.add(sphere!());

        let mut branch = SceneTree::default();
        branch.add(cube!());
        branch.add(plane!());

        tree.add(branch);

        let vec = tree.flatten_scene();
        assert_eq!(3, vec.len());
    }
}

#[cfg(test)]
mod flatten_matrix_tests {
    use super::*;

    use crate::transform::Transform;
    use crate::{cube, cylinder, sphere};
    use math::{degrees, matrix4x4};

    #[test]
    fn combine_matrix_from_parent() {
        let r = matrix4x4!(shear(2.0, 3.0, 4.0, 5.0, 6.0, 7.0));
        let a = matrix4x4!(scale_all(2.0));
        let b = matrix4x4!(translation(1.0, 2.0, 3.0));
        let c = matrix4x4!(rotation_x(degrees!(90)));
        let d = matrix4x4!(rotation_y(degrees!(45)));

        let mut root = SceneTree::new(r);
        root.add(sphere!(matrix: a));

        let mut branch = SceneTree::new(b);
        branch.add(cube!(matrix: c));

        root.add(branch);
        root.add(cylinder!(matrix: d));

        let vec = root.flatten_scene();

        // root (r)
        //   - tree (i)
        //       - sphere (a) => r * a
        //   - tree (b)
        //       - cube (c) => r * c * b
        //   - cylinder (d) => r * d
        //

        assert_eq!(3, vec.len());
        assert_eq!(Transform::new(r * a), vec.get(0).unwrap().transform);
        assert_eq!(Transform::new(r * b * c), vec.get(1).unwrap().transform);
        assert_eq!(Transform::new(r * d), vec.get(2).unwrap().transform);
    }
}

#[cfg(test)]
mod material_override_tests {
    use super::*;

    use crate::material::Material;

    use crate::{cube, cylinder, scene, sphere};

    #[test]
    fn override_material() {
        let vec = scene!(
            +sphere!();
            +scene!(
                material_override: Material::air();
                +cube!();
                +scene!(
                    +cube!();
                );
            );
            +cylinder!();
        )
        .flatten_scene();

        assert_eq!(4, vec.len());
        assert_eq!(Material::default(), vec.get(0).unwrap().material);
        assert_eq!(Material::air(), vec.get(1).unwrap().material);
        assert_eq!(Material::air(), vec.get(2).unwrap().material);
        assert_eq!(Material::default(), vec.get(3).unwrap().material);
    }

    #[test]
    fn override_material_with_bounding_volume() {
        let vec = scene!(
            +sphere!();
            +scene!(
                material_override: Material::air();
                bounding_volume: cube!();
                +cube!();
                +scene!(
                    +cube!();
                );
            );
            +cylinder!();
        )
        .flatten_scene();

        assert_eq!(5, vec.len());
        assert_eq!(Material::default(), vec.get(0).unwrap().material);
        // index 1 is the BV
        assert_eq!(Material::air(), vec.get(2).unwrap().material);
        assert_eq!(Material::air(), vec.get(3).unwrap().material);
        assert_eq!(Material::default(), vec.get(4).unwrap().material);
    }

    #[test]
    fn double_override_material() {
        let vec = scene!(
            +sphere!();
            +scene!(
                material_override: Material::air();
                +cube!();
                +scene!(
                    // second override
                    material_override: Material::glass();
                    +cube!();
                );
            );
            +cylinder!();
        )
        .flatten_scene();

        assert_eq!(4, vec.len());
        assert_eq!(Material::default(), vec.get(0).unwrap().material);
        assert_eq!(Material::air(), vec.get(1).unwrap().material);
        assert_eq!(Material::glass(), vec.get(2).unwrap().material);
        assert_eq!(Material::default(), vec.get(3).unwrap().material);
    }

    #[test]
    fn override_material_with_csg() {
        let vec = scene!(
            +sphere!();
            +scene!(
                material_override: Material::air();
                +cube!() + sphere!();
            );
            +cylinder!();
        )
        .flatten_scene();

        assert_eq!(5, vec.len());
        assert_eq!(Material::default(), vec.get(0).unwrap().material);
        // index 1 is the CSG node
        assert_eq!(Material::air(), vec.get(2).unwrap().material);
        assert_eq!(Material::air(), vec.get(3).unwrap().material);
        assert_eq!(Material::default(), vec.get(4).unwrap().material);
    }
}
