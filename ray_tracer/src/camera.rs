use crate::ray;
use crate::rays::Ray;
use math::matrix::matrix_4x4::Matrix4x4;
use math::{Angle, point, vector};

struct Camera {
    view_port: (usize, usize),
    aspect_ratio: f32,
    fov_y: Angle,
    transform: Matrix4x4,
    invert_transform: Matrix4x4,
    canvas_size: (f32, f32),
}

impl Camera {
    pub fn new(view_port: (usize, usize), fov_y: Angle) -> Self {
        let aspect_ratio = view_port.0 as f32 / view_port.1 as f32;
        let canvas_height = (fov_y.to_radians() / 2.).tan() * 2.;
        let canvas_width = canvas_height * aspect_ratio;
        Self {
            view_port,
            aspect_ratio,
            fov_y,
            transform: Matrix4x4::identity(),
            invert_transform: Matrix4x4::identity(),
            canvas_size: (canvas_width, canvas_height),
        }
    }

    pub fn ray_for_pixel(&self, x_y: (usize, usize)) -> Ray {
        let (x, y) = x_y;
        let (width, height) = self.canvas_size;

        let x_normalized = (x as f32 + 0.5) / self.width() - 0.5;
        let y_normalized = (y as f32 + 0.5) / self.height() - 0.5;

        let y1 = (-y_normalized) * height;
        let x1 = (-x_normalized) * width;

        let point = (self.invert_transform * point!(x1, y1, -1)).force_point();
        let origin = (self.invert_transform * point!(0, 0, 0)).force_point();
        let direction = (point - origin).normalize();

        ray!(origin, direction.to_vector())
    }

    fn aspect(&self) -> f32 {
        self.width() / self.height()
    }

    fn width(&self) -> f32 {
        self.view_port.0 as f32
    }

    fn height(&self) -> f32 {
        self.view_port.1 as f32
    }

    pub fn set_transform(&mut self, transform: Matrix4x4) {
        self.invert_transform = transform.invert().expect("unable to invert transform");
        self.transform = transform;
    }
}

#[cfg(test)]
mod camera_tests {
    use super::*;
    use math::tuple::point::Point;
    use math::{degrees, point, vector};

    #[test]
    fn new_camera() {
        let camera = Camera::new((160, 120), degrees!(45));
        assert_eq!(160, camera.view_port.0);
        assert_eq!(120, camera.view_port.1);
        assert_eq!(45., camera.fov_y.to_degrees());
    }

    #[test]
    fn ray_for_center_of_view() {
        let camera = Camera::new((201, 101), degrees!(90));
        let ray = camera.ray_for_pixel((100, 50));
        assert_eq!(ray.origin, point!(0, 0, 0));
        assert_eq!(ray.direction, vector!(0, 0, -1));
    }

    #[test]
    fn ray_for_top_left_of_view() {
        let camera = Camera::new((101, 201), degrees!(90));
        let ray = camera.ray_for_pixel((0, 0));
        assert_eq!(ray.origin, point!(0, 0, 0));
        assert_eq!(ray.direction, vector!(0.3325932, 0.6651864, -0.66851234));
    }

    #[test]
    fn ray_for_transformed_view() {
        let mut camera = Camera::new((101, 201), degrees!(90));
        camera.set_transform(
            Matrix4x4::rotation_y(degrees!(45).to_radians()).pre_translation(0., -2., 5.),
        );
        let ray = camera.ray_for_pixel((50, 100));
        assert_eq!(ray.origin, point!(0, 2, -5.0000005));
        assert_eq!(ray.direction, vector!(0.70710665, 0, -0.7071069));
    }
}
