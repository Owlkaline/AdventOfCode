use std::fs;

fn main() {
    println!("Hello, world!");

    let filename = "example";

    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            let parts: Vec<String> = line.split_ascii_whitespace().collect();
            for entry in parts {}
        }
    }
}
