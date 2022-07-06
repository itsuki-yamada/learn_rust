use std::fs::read_to_string;

fn main() {
    run_cat();
}

fn run_cat() {
    let path = "./src/main.rs";
    match read_to_string(path) {
        Ok(content) => println!("{}", content),
        Err(reason) => println!("{}", reason),
    }
}
