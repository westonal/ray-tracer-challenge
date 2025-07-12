use math::tuple::color::Color;
use ray_tracer::canvas::Canvas;
use ray_tracer::material::Material;

mod image_buffer_canvas;
mod png_write;
pub mod test_scenes;
mod threaded_canvas;

fn fill_all_with_gradient<C: Canvas<Color>>(canvas: &mut C) {
    let size = canvas.size();
    for y in 0..size.height() {
        for x in 0..size.width() {
            let color = (
                (x as f32) / (size.width() as f32),
                (y as f32) / (size.height() as f32),
                0.0,
                1.0,
            )
                .into();
            canvas.write_color(x, y, color);
        }
    }
}
