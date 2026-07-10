use std::fs;

mod interpreter;
use interpreter::Interpreter;

use crate::interpreter::InterpOptions;

const FILE_PATH: &str = "./bf.txt";
const DEFAULT_TAPE_SIZE: usize = 30_000;

fn main() {
    let raw_code = fs::read_to_string(FILE_PATH).expect("Error reading the file");
    let mut interp = Interpreter::new(raw_code.trim().to_string(), DEFAULT_TAPE_SIZE);
    //interp.exec(InterpOptions::default());
    interp.exec(InterpOptions {
        delay_ms: 500,
        verbose: true,
    });

    println!("{interp}")
}
