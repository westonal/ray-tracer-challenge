use crate::canvas::Canvas;
use crate::tuple::Tuple;

mod canvas;
mod tuple;

fn main() {
    let vector = Tuple::point(1.0, 2.0, 3.0);
    let mut speed = Tuple::vector(2.0, 1.0, -0.1);
    let acceleration = Tuple::vector(0.0, 0.01, 0.0);
    for i in 0..100 {
        println!("{}", vector + speed * i as f32)
    }

    let mut canvas = Canvas::new(200, 200);
    for i in 0..50 {
        let point = vector + speed * i as f32;
        println!("{}: {}", i, point);
        canvas.write_color(point.x as u32, point.y as u32);
        speed = speed + acceleration;
    }
    canvas.save();
}
