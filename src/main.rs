use crate::tuple::Tuple;

mod tuple;

fn main() {
    let vector = Tuple::vector(1.0, 2.0, 3.0);
    println!("Hello, world! {vector}");
}
