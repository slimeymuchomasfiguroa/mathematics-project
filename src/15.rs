use std::num::NonZeroUsize;
fn main() {
    let n = NonZeroUsize::new(10).unwrap();
    println!("{}", n);
}
