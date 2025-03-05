use std::fs;

fn main() {
    let mut file = fs::File::open("numbers.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for line in contents.lines() {
        if line.contains("a") || line.contains("e") {
            println!("{}", line);
        }
    }
}
