use std::fmt;

#[derive(Clone)]
pub struct InterpreterState {
    pub code: Vec<char>,
    // last_action: Option<char>,
    pub tape: Vec<u8>,
    pub ptr: usize,
    pub step: usize,
    pub output: Vec<u8>,
}

impl InterpreterState {
    pub fn new(code: Vec<char>, tape_size: usize) -> Self {
        Self {
            code,
            tape: vec![0u8; tape_size],
            ptr: 0,
            step: 0,
            output: vec![],
        }
    }

    pub fn tape(&self) -> &Vec<u8> {
        &self.tape
    }

    pub fn output(&self) -> &Vec<u8> {
        &self.output
    }

    pub fn code(&self) -> &Vec<char> {
        &self.code
    }
}

// #[derive(Debug)]
pub struct Interpreter {
    pub state: InterpreterState,
    // pub code: Vec<char>,
    pub bracklet_map: Vec<usize>,
    // pub tape: Vec<u8>,
    // pub ptr: usize,
    // pub step: usize,
    waiting_for_input: bool,
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pointer position : {}\ncurrent step : {}\nBF code : '{}'\ntape [..100] : {:?}",
            self.state.ptr,
            self.state.step,
            self.state.code.iter().collect::<String>(),
            &(self.state.tape)[..100],
        )
    }
}

pub enum Effect {
    AskInput,
    Pass,
    End,
}

impl Interpreter {
    pub fn new(raw_code: String, tape_size: usize) -> Self {
        let mut interp = Self {
            state: InterpreterState::new(raw_code.chars().collect(), tape_size),
            bracklet_map: vec![0usize; raw_code.len()],
            waiting_for_input: false,
        };
        match interp.build_bracklet_map() {
            Ok(()) => {}
            Err(e) => println!("{}", e),
        };
        interp
    }

    pub fn reset(&mut self) {
        self.state = InterpreterState::new(self.state.code.clone(), self.state.tape.len());
        self.build_bracklet_map();
    }

    fn build_bracklet_map(&mut self) -> Result<(), String> {
        let mut stack = Vec::new();
        for (i, char) in self.state.code.iter().enumerate() {
            match char {
                '[' => stack.push(i),
                ']' => {
                    let origin = match stack.pop() {
                        Some(indice) => indice,
                        None => {
                            return Err(format!(
                                "Closed bracklet at position {} without corresponding '['.",
                                i
                            ));
                        }
                    };
                    self.bracklet_map[i] = origin;
                    self.bracklet_map[origin] = i;
                }
                _ => {}
            }
        }
        match stack.pop() {
            Some(indice) => {
                return Err(format!(
                    "Opening bracklet found at position {} was never closed.",
                    indice
                ));
            }
            None => {}
        }
        Ok(())
    }

    pub fn exec_current_step(&mut self, entry: Option<u8>) -> Option<Effect> {
        if self.state.step == self.state.code.len() {
            return Some(Effect::End);
        }
        if self.waiting_for_input {
            self.state.tape[self.state.ptr] = entry.unwrap_or(0);
            // println!("Input received : {}", entry.unwrap_or(0));
            self.waiting_for_input = false;
            self.state.step += 1;
            return None;
        }

        match self.state.code[self.state.step] {
            '>' => self.state.ptr += 1,
            '<' => self.state.ptr -= 1,
            '+' => {
                self.state.tape[self.state.ptr] = self.state.tape[self.state.ptr].wrapping_add(1)
            }
            '-' => {
                self.state.tape[self.state.ptr] = self.state.tape[self.state.ptr].wrapping_sub(1)
            }
            '[' => {
                if self.state.tape[self.state.ptr] == 0 {
                    self.state.step = self.bracklet_map[self.state.step];
                }
            }
            ']' => self.state.step = self.bracklet_map[self.state.step] - 1,
            '.' => self.state.output.push(self.state.tape[self.state.ptr]),
            ',' => {
                self.waiting_for_input = true;
                return Some(Effect::AskInput);
            }
            _ => {
                self.state.step += 1;
                return Some(Effect::Pass);
            }
        };
        self.state.step += 1;
        None
    }

    pub fn state(&self) -> &InterpreterState {
        &self.state
    }
}
