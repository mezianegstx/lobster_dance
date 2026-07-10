use std::{fs, thread, time::Duration};

mod interpreter;
use interpreter::Interpreter;

mod cli;

const FILE_PATH: &str = "./bf.txt";
const DEFAULT_TAPE_SIZE: usize = 30_000;

#[derive(PartialEq)]
enum Verbose {
    Mute,
    CurrentStep,
    AllSteps,
}

struct ExecOptions {
    delay_ms: u64,
    verbose: Verbose,
}

impl ExecOptions {
    pub fn default() -> Self {
        Self {
            delay_ms: 0,
            verbose: Verbose::CurrentStep,
        }
    }
}

fn exec(interp: &mut Interpreter, options: ExecOptions) {
    while interp.step < interp.code.len() {
        let erase = options.verbose != Verbose::AllSteps;
        if options.verbose != Verbose::Mute {
            cli::print_step_by_step(interp.tape(), interp.action(), erase);
        }
        interp.exec_current_step();
        thread::sleep(Duration::from_millis(options.delay_ms));
    }
    if options.verbose == Verbose::CurrentStep {
        interp.step -= 1;
        cli::print_step_by_step(interp.tape(), interp.action(), false);
    }
}

fn main() {
    let raw_code = fs::read_to_string(FILE_PATH).expect("Error reading the file");
    let mut interp = Interpreter::new(raw_code.trim().to_string(), DEFAULT_TAPE_SIZE);
    // exec(&mut interp, ExecOptions::default());
    exec(
        &mut interp,
        ExecOptions {
            delay_ms: 50,
            verbose: Verbose::AllSteps,
        },
    );
    println!("{interp}");
}
