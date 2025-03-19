use std::f32::consts;

fn main() {
    let mut x = 0f32;
    for i in 1..=5 {
        x += consts::PI / (i as f32);
    }
    println!("The value of pi is: {}", x);
}
