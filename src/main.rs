use std::{fs, thread, time::Duration};

mod interpreter;
use interpreter::Interpreter;

mod cli;
use cli::CommandLineInterface;

use crate::interpreter::Effect;

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
            verbose: Verbose::Mute,
        }
    }
}

struct Controller {
    options: ExecOptions,
    model: Interpreter,
    view: CommandLineInterface,
}

impl Controller {
    fn exec(&mut self) {
        println!("\n");
        while self.model.step < self.model.code.len() {
            let erase = self.options.verbose != Verbose::AllSteps;
            if self.options.verbose != Verbose::Mute {
                CommandLineInterface::print_step_by_step(
                    self.model.tape(),
                    self.model.action(),
                    erase,
                );
            };
            let mut entry: Option<u8> = None;
            match self.model.exec_current_step(entry) {
                Some(effect) => match effect {
                    Effect::AskInput => entry = Some(50),
                    Effect::Output(octet) => print!("{}", octet as char),
                },
                None => {}
            }
            thread::sleep(Duration::from_millis(self.options.delay_ms));
        }
        if self.options.verbose == Verbose::CurrentStep {
            self.model.step -= 1;
            CommandLineInterface::print_step_by_step(self.model.tape(), self.model.action(), false);
        }
        println!("\n");
    }
}

fn main() {
    let raw_code = fs::read_to_string(FILE_PATH).expect("Error reading the file");
    let mut controller = Controller {
        // options: ExecOptions {
        //     delay_ms: 50,
        //     verbose: Verbose::CurrentStep,
        // },
        options: ExecOptions::default(),
        model: Interpreter::new(raw_code.trim().to_string(), DEFAULT_TAPE_SIZE),
        view: CommandLineInterface {},
    };
    controller.exec();

    println!("{}", controller.model);
}
