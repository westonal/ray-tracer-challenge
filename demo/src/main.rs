use demo::test_scenes::RenderTestScene;
use demo::test_scenes::chapter7_scene::Chapter7Scene;
use demo::test_scenes::chess_pawn::Pawn;
use demo::test_scenes::chess_queen::Queen;
use demo::test_scenes::cube_of_spheres::CubeOfSpheres;
use demo::test_scenes::cubes::Cubes;
use demo::test_scenes::cylinders::Cylinders;
use demo::test_scenes::glass_sphere_with_air::GlassSphereWithAir;
use demo::test_scenes::grid::Grid;
use demo::test_scenes::teapot::Teapot;
use demo::test_scenes::triangles::Triangles;
use ray_tracer::canvas::Size;

fn main() {
    let size = Size::HD_720P;
    Teapot::render_scene(size);
    Pawn::render_scene(size);
    Queen::render_scene(size);
    Triangles::render_scene(size);
    Chapter7Scene::render_scene(size);
    GlassSphereWithAir::render_scene(size);
    Cubes::render_scene(size);
    Cylinders::render_scene(size);
    CubeOfSpheres::render_scene(size);
    Grid::render_scene(size);
}
