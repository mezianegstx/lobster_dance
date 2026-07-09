use std::fs;

const FILE_PATH: &str = "./bf.txt";

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string(FILE_PATH);
    match contents {
        Ok(ref content) => println!("{}", content),
        Err(_) => println!("Eroor while reading th file"),
    }
    println!("{:#?}", contents);
}
