use crate::canvas::Canvas;
use crate::image_buffer_canvas::ImageBufferCanvas;
use math::tuple::Tuple;
use math::tuple::color::Color;

mod canvas;
mod image_buffer_canvas;

fn main() {
    let mut canvas = ImageBufferCanvas::new(200, 200);
    fill_all_with_gradient(&mut canvas);
    example(&mut canvas);
    canvas.save_png("out.png");
}

fn fill_all_with_gradient<C: Canvas<Color>>(canvas: &mut C) {
    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            let color = (
                (x as f32) / (canvas.width() as f32),
                (y as f32) / (canvas.height() as f32),
                0.0,
                1.0,
            )
                .into();
            canvas.write_color(x, y, color);
        }
    }
}

fn example<C: Canvas<Color>>(canvas: &mut C) {
    let vector = Tuple::point(10.0, 20.0, 3.0);
    let mut speed = Tuple::vector(2.0, -2.0, -0.1);
    let acceleration = Tuple::vector(0.0, 0.05, 0.0);
    for i in 0..100 {
        println!("{}", vector + speed * i as f32)
    }
    for i in 0..50 {
        let point = vector + speed * i as f32;
        let color = (0.5, (i as f32) / 50.0, 1.0 - (i as f32) / 25.0, 1.0).into();
        println!("{}: {} = {}", i, point, color);
        canvas.write_color(point.x as u32, point.y as u32, color);
        speed = speed + acceleration;
    }
}
