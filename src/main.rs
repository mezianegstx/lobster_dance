#![allow(warnings)]

use std::{
    fs,
    io::Read,
    thread,
    time::{Duration, Instant},
};

mod interpreter;
use crate::interpreter::InterpreterState;
use interpreter::Interpreter;

mod cli;
use cli::CommandLineInterface;

use crate::interpreter::Effect;

const FILE_PATH: &str = "./bf_files/HelloWord.bf";
const DEFAULT_TAPE_SIZE: usize = 30_000;

struct ExecOptions {
    delay_ms: u64,
    tape_size: usize,
}

enum FrontendEvent {
    Run,
    CharProvided(char),
    CharTyped(char),
    Resized,
    Quit,
    None,
}

impl ExecOptions {
    pub fn default() -> Self {
        Self {
            delay_ms: 0,
            tape_size: DEFAULT_TAPE_SIZE,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Mode {
    Edition,
    Execution(ExecutionState),
}

#[derive(Copy, Clone, PartialEq)]
enum ExecutionState {
    Running,
    Paused,
    AskingInput,
}

struct Controller {
    options: ExecOptions,
    model: Interpreter,
    view: CommandLineInterface,
    mode: Mode,
}

impl Controller {
    fn exec(&mut self) {
        let mut entry: Option<u8> = None;
        let state = self.model.state();
        let mut last_step = Instant::now();
        loop {
            self.view.render(self.model.state(), self.mode);

            match self.view.poll(self.mode) {
                FrontendEvent::Run => {
                    self.model.reset();
                    self.mode = Mode::Execution(ExecutionState::Running);
                }
                FrontendEvent::CharProvided(c) => {
                    entry = Some(c as u8);
                    self.mode = Mode::Execution(ExecutionState::Running);
                }
                FrontendEvent::CharTyped(c) => {}
                FrontendEvent::Resized => {}
                FrontendEvent::None => {}
                FrontendEvent::Quit => break,
            }
            if self.mode == Mode::Execution(ExecutionState::Running)
                && last_step.elapsed().as_millis() as u64 >= self.options.delay_ms
            {
                match self.model.exec_current_step(entry) {
                    Some(effect) => match effect {
                        Effect::AskInput => {
                            self.mode = Mode::Execution(ExecutionState::AskingInput)
                        }
                        Effect::Pass => continue,
                        Effect::End => self.mode = Mode::Edition,
                    },
                    None => {}
                }
                last_step = Instant::now();
            }
            // thread::sleep(Duration::from_millis(self.options.delay_ms));
        }
    }
}

fn main() {
    let raw_code = fs::read_to_string(FILE_PATH).expect("Error reading the file");
    let mut controller = Controller {
        options: ExecOptions {
            delay_ms: 10,
            tape_size: 100,
        },
        // options: ExecOptions::default(),
        model: Interpreter::new(raw_code.trim().to_string(), DEFAULT_TAPE_SIZE),
        view: CommandLineInterface::new(),
        mode: Mode::Edition,
    };

    controller.exec();
    thread::sleep(Duration::from_millis(1000));
}
