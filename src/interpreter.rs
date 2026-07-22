use std::fmt;

use crossterm::cursor;

pub struct Code {
    lines: Vec<Vec<char>>,
}

impl Code {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn from(raw_code: Vec<char>) -> Self {
        let mut s = Self::new();
        s.indent(&raw_code);
        s
    }

    pub fn indent(&mut self, code: &Vec<char>) {
        self.lines = Vec::new();
        let mut dept: usize = 0;
        let mut line: Vec<char> = Vec::new();
        for &c in code {
            if c == '[' {
                dept += 1;
                line.push(c);
                line.push(' ');
                self.lines.push(line);
                line = vec![' '; 2 * dept];
            } else if c == '\n' {
                line.push(' ');
                self.lines.push(line);
                line = vec![' '; 2 * dept];
            } else if c == ']' {
                line.push(' ');
                self.lines.push(line);
                dept -= 1;
                line = vec![' '; 2 * dept];
                line.push(c);
                line.push(' ');
                self.lines.push(line);
                line = vec![' '; 2 * dept];
            } else {
                line.push(c);
            }
        }
        // &self.lines
    }

    pub fn insert(&mut self, mut pos: usize, c: char) -> Result<(), String> {
        for line in &mut self.lines {
            if pos < line.len() {
                line.insert(pos, c);
                return Ok(());
            }
            pos -= line.len();
        }
        Err(format!("Index out of bound"))
    }

    pub fn remove(&mut self, mut pos: usize) -> Result<(), String> {
        for line in &mut self.lines {
            if pos < line.len() {
                line.remove(pos);
                return Ok(());
            }
            pos -= line.len();
        }
        Err(format!("Index out of bound"))
    }

    pub fn lines(&self) -> &Vec<Vec<char>> {
        &self.lines
    }

    pub fn code(&self) -> Vec<char> {
        let mut code: Vec<char> = Vec::new();
        for line in self.lines.clone() {
            code.extend(line);
            // code.extend(vec!['\n']);
        }
        code
    }
}

#[derive(Clone)]
pub struct InterpreterState {
    // pub code: Vec<char>,
    // last_action: Option<char>,
    pub tape: Vec<u8>,
    pub ptr: usize,
    pub step: usize,
    pub output: Vec<u8>,
}

impl InterpreterState {
    pub fn new(tape_size: usize) -> Self {
        Self {
            // code,
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

    // pub fn code(&self) -> &Vec<char> {
    //     &self.code
    // }
}

// #[derive(Debug)]
pub struct Interpreter {
    pub state: InterpreterState,
    pub code: Code,
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
            self.code.code().iter().collect::<String>(),
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
            state: InterpreterState::new(tape_size),
            code: Code::from(raw_code.chars().collect()),
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
        self.state = InterpreterState::new(self.state.tape.len());
        self.code = Code::from(self.code.code());
        self.build_bracklet_map();
    }

    pub fn code_insert(&mut self, pos: usize, c: char) {
        self.state.code.insert(pos, c);
    }

    pub fn code_del(&mut self, pos: usize) {
        self.state.code.remove(pos);
    }

    pub fn indent(&mut self, mut cursor_pos: usize) -> usize {
        let mut dept: usize = 0;
        let old_code = self.state.code.clone();
        self.state.code = Vec::new();
        // let mut line: Vec<char> = Vec::new();
        let mut offset: usize = 0;
        for (i, &c) in old_code.iter().enumerate() {
            if c == '[' {
                dept += 1;
                self.state.code.extend(vec![c, '\n']);
                self.state.code.extend(vec![' '; 2 * dept]);
                offset += 2 * (dept + 1)
            } else if c == '\n' {
                self.state.code.push('\n');
                self.state.code.extend(vec![' '; 2 * dept]);
            } else if c == ']' {
                self.state.code.push('\n');
                dept -= 1;
                self.state.code.extend(vec![' '; 2 * dept]);
                self.state.code.extend(vec![c, '\n']);
                self.state.code.extend(vec![' '; 2 * dept]);
            } else {
                self.state.code.push(c);
            }
        }
        cursor_pos
    }

    fn clean(&mut self, mut cursor_pos: usize) -> usize {
        let old_code = self.state.code.clone();
        let mut space = false;
        let mut enter = false;
        let mut offset: usize = 0;
        for (i, &c) in old_code.iter().enumerate() {
            match c {
                ' ' => {
                    if space {
                        self.state.code.remove(i);
                        offset += 1;
                    } else {
                        space = true;
                    }
                }
                '\n' => {
                    if enter {
                        self.state.code.remove(i);
                        offset += 1;
                    } else {
                        enter = true;
                    }
                }
                _ => {
                    space = false;
                    enter = false;
                } // if i==cursor_pos {
                  //     cursor_pos +=
                  //     }
            }
        }
        cursor_pos
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
