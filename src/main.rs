use std::{fs, thread, time::Duration};

mod interpreter;
use interpreter::Interpreter;

mod cli;

const FILE_PATH: &str = "./bf.txt";
const DEFAULT_TAPE_SIZE: usize = 30_000;

struct ExecOptions {
    delay_ms: u64,
    verbose: bool,
}

impl ExecOptions {
    pub fn default() -> Self {
        Self {
            delay_ms: 0,
            verbose: false,
        }
    }
}

fn exec(interp: &mut Interpreter, options: ExecOptions) {
    while interp.step < interp.code.len() {
        interp.exec_current_step();
        thread::sleep(Duration::from_millis(options.delay_ms));
        cli::print_step(interp.tape());
    }
}

fn main() {
    let raw_code = fs::read_to_string(FILE_PATH).expect("Error reading the file");
    let mut interp = Interpreter::new(raw_code.trim().to_string(), DEFAULT_TAPE_SIZE);
    exec(&mut interp, ExecOptions::default());

    println!("{interp}");
}
