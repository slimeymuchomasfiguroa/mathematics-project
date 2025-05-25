use std::fs::File;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("mathematical_expression.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        print!("{}", line.unwrap());
    }

    Ok(())
}
