use crate::canvas::Canvas;
use crate::image_buffer_canvas::ImageBufferCanvas;
use crate::tuple::color::Color;
use crate::tuple::Tuple;

mod canvas;
mod image_buffer_canvas;
mod tuple;

fn main() {
    let mut canvas = ImageBufferCanvas::new(200, 200);
    example(&mut canvas);
    canvas.save_png("out.png");
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
