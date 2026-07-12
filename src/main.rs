#![allow(warnings)]

use std::{fs, io::Read, thread, time::Duration};

mod interpreter;
use interpreter::Interpreter;

mod cli;
use cli::CommandLineInterface;

use crate::interpreter::Effect;

const FILE_PATH: &str = "./bf_files/HelloWord.bf";
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

#[derive(Copy, Clone, PartialEq)]
enum Mode {
    Edition,
    Execution,
}

struct Controller {
    options: ExecOptions,
    model: Interpreter,
    view: CommandLineInterface,
    mode: Mode,
}

impl Controller {
    fn exec(&mut self) {
        let mut buf = [0u8, 1];
        let mut entry: Option<u8> = None;
        let state = self.model.state();
        while self.model.state().step < self.model.state().code.len() {
            match self.model.exec_current_step(entry) {
                Some(effect) => match effect {
                    Effect::AskInput => match std::io::stdin().read_exact(&mut buf) {
                        Ok(()) => entry = Some(buf[0] as u8),
                        Err(_) => entry = None,
                    },
                    Effect::Output(octet) => print!("{}", octet as char),
                    Effect::Pass => continue,
                },
                None => {}
            }
            // let erase = self.options.verbose != Verbose::AllSteps;
            // if self.options.verbose != Verbose::Mute {
            //     CommandLineInterface::print_step_by_step(
            //         self.model.tape(),
            //         self.model.action(),
            //         erase,
            //     );
            // };
            self.view.render(self.model.state(), self.mode);
            thread::sleep(Duration::from_millis(self.options.delay_ms));
        }
        // if self.options.verbose == Verbose::CurrentStep {
        //     self.model.step -= 1;
        //     CommandLineInterface::print_step_by_step(self.model.tape(), self.model.action(), false);
        // }
        // println!("\n");
    }
}

fn main() {
    let raw_code = fs::read_to_string(FILE_PATH).expect("Error reading the file");
    let mut controller = Controller {
        options: ExecOptions {
            delay_ms: 50,
            verbose: Verbose::Mute,
        },
        // options: ExecOptions::default(),
        model: Interpreter::new(raw_code.trim().to_string(), DEFAULT_TAPE_SIZE),
        view: CommandLineInterface::new(),
        mode: Mode::Execution,
    };
    controller.exec();

    // let state = InterpreterState::new(vec![0u8; 100], vec!['+', '+', '+', '>', '-', '0', '<'], 1);

    // controller.view.render(&state, controller.mode);
    thread::sleep(Duration::from_millis(10000));
    // println!("{}", controller.model);
}

use crate::interpreter::InterpreterState;
