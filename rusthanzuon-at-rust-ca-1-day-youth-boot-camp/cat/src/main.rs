use std::fs::read_to_string;

fn main() {
    match std::env::args().nth(1) {
        Some(path) => run_cat(path),
        None => println!("No path is specifed!"),
    }
}

fn run_cat(path: String) {
    match read_to_string(path) {
        Ok(content) => println!("{}", content),
        Err(reason) => println!("{}", reason),
    }
}
