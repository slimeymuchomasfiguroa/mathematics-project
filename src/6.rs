fn main() {
    let mut rng = rand::thread_rng();
    println!("Random number between 1 and 10: {}", rng.gen_range(1..=10));
}
