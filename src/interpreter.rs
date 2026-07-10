use std::fmt;

#[derive(Debug)]
pub struct Interpreter {
    pub code: Vec<char>,
    pub bracklet_map: Vec<usize>,
    pub tape: Vec<u8>,
    pub ptr: usize,
    pub step: usize,
    waiting_for_input: bool,
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pointer position : {}\ncurrent step : {}\nBF code : '{}'\ntape [..100] : {:?}",
            self.ptr,
            self.step,
            self.code.iter().collect::<String>(),
            &(self.tape)[..100],
        )
    }
}

pub enum Effect {
    Output(u8),
    AskInput,
}

impl Interpreter {
    pub fn new(raw_code: String, tape_size: usize) -> Self {
        let mut interp = Self {
            code: raw_code.chars().collect(),
            bracklet_map: vec![0usize; raw_code.len()],
            tape: vec![0u8; tape_size],
            ptr: 0,
            step: 0,
            waiting_for_input: false,
        };
        match interp.build_bracklet_map() {
            Ok(()) => {}
            Err(e) => println!("{}", e),
        };
        interp
    }

    fn build_bracklet_map(&mut self) -> Result<(), String> {
        let mut stack = Vec::new();
        for (i, char) in self.code.iter().enumerate() {
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
        if self.waiting_for_input {
            self.tape[self.ptr] = entry.unwrap_or(0);
            println!("Input received : {}", entry.unwrap_or(0));
            self.waiting_for_input = false;
            self.step += 1;
            return None;
        }

        match self.code[self.step] {
            '>' => self.ptr += 1,
            '<' => self.ptr -= 1,
            '+' => self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(1),
            '-' => self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(1),
            '[' => {
                if self.tape[self.ptr] == 0 {
                    self.step = self.bracklet_map[self.step];
                }
            }
            ']' => self.step = self.bracklet_map[self.step] - 1,
            '.' => {
                self.step += 1;
                return Some(Effect::Output(self.tape[self.ptr]));
            }
            ',' => {
                self.waiting_for_input = true;
                return Some(Effect::AskInput);
            }
            _ => {}
        };
        self.step += 1;
        None
    }

    pub fn tape(&self) -> &[u8] {
        &self.tape
    }
    pub fn action(&self) -> char {
        self.code[self.step]
    }
}
