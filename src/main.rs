use crate::tuple::Tuple;

mod tuple;

fn main() {
    let vector = Tuple::point(1.0, 2.0, 3.0);
    let speed = Tuple::vector(0.1, 0.2, -0.1);
    for i in 0..100 {
        println!("{}", vector + speed * i as f32)
    }
}
