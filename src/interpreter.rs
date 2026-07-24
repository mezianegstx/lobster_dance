use std::fmt;

use crossterm::cursor;

pub struct Code {
    code: Vec<char>,
}

impl Code {
    fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn from_string(raw_code: String) -> Self {
        let mut c = Self {
            code: raw_code.chars().collect(),
        };
        c.indent();
        c
        // let mut s = Self::new();
        // let mut dept: usize = 0;
        // let mut line: Vec<char> = Vec::new();
        // for c in raw_code.trim().chars() {
        //     if c == '[' {
        //         dept += 1;
        //         line.push(c);
        //         line.push(' ');
        //         s.lines.push(line);
        //         line = vec![' '; 2 * dept];
        //     } else if c == '\n' {
        //         line.push(' ');
        //         s.lines.push(line);
        //         line = vec![' '; 2 * dept];
        //     } else if c == ']' {
        //         line.push(' ');
        //         s.lines.push(line);
        //         dept -= 1;
        //         line = vec![' '; 2 * dept];
        //         line.push(c);
        //         line.push(' ');
        //         s.lines.push(line);
        //         line = vec![' '; 2 * dept];
        //     } else {
        //         line.push(c);
        //     }
        // }
        // s
    }

    // fn build_bracklet_map(&mut self) -> Result<(), String> {
    //     let mut stack = Vec::new();
    //     for (i, char) in self.code.code().iter().enumerate() {
    //         match char {
    //             '[' => stack.push(i),
    //             ']' => {
    //                 let origin = match stack.pop() {
    //                     Some(indice) => indice,
    //                     None => {
    //                         return Err(format!(
    //                             "Closed bracklet at position {} without corresponding '['.",
    //                             i
    //                         ));
    //                     }
    //                 };
    //                 self.bracklet_map[i] = origin;
    //                 self.bracklet_map[origin] = i;
    //             }
    //             _ => {}
    //         }
    //     }
    //     match stack.pop() {
    //         Some(indice) => {
    //             return Err(format!(
    //                 "Opening bracklet found at position {} was never closed.",
    //                 indice
    //             ));
    //         }
    //         None => {}
    //     }
    //     Ok(())
    // }

    fn clean(&mut self) {
        //let old_code = self.state.code.clone();
        let mut space = false;
        let mut enter = false;
        //let mut offset: usize = 0;
        // for line in &mut self.code {
        self.code.retain(|&c|
        // for (i, c) in &mut line.iter_mut().enumerate() {
            match c {
                ' ' => {
                    if space {
                        // line.remove(i);
                        // offset += 1;
                        false
                    } else {
                        space = true;
                        true
                    }
                }
                '\n' => {
                    if enter {
                        // line.remove(i);
                        // offset += 1;
                        false
                    } else {
                        enter = true;
                        true
                    }
                }
                _ => {
                    space = false;
                    enter = false;
                    true
                } // if i==cursor_pos {
                    //     cursor_pos +=
                    //     }
            }) //}
        // }
        // cursor_pos
    }

    pub fn indent(&mut self) {
        self.clean();
        let mut old_code = self.code.clone();
        self.code = Vec::new();
        let mut dept = 0;
        for c in old_code {
            if c == '[' {
                dept += 1;
                self.code.extend(vec![c, '\n']);
                // self.code.push(' ');
                // self.code.push('\n');
                self.code.extend(vec![' '; 2 * dept]);
            } else if c == '\n' {
                self.code.extend(vec!['\n']);
                self.code.extend(vec![' '; 2 * dept]);
            } else if c == ']' {
                self.code.extend(vec!['\n']);
                dept -= 1;
                self.code.extend(vec![' '; 2 * dept]);
                self.code.extend(vec![c, '\n']);
                self.code.extend(vec![' '; 2 * dept]);
            } else {
                self.code.push(c);
            }
        }
    }

    pub fn insert(&mut self, mut offset: usize, c: char) {
        // -> Result<(), String> {
        self.code.insert(offset, c);
        // for line in &mut self.lines {
        //     if pos < line.len() {
        //         line.insert(pos, c);
        //         return Ok(());
        //     }
        //     pos -= line.len();
        // }
        // Err(format!("Index out of bound"))
    }

    pub fn remove(&mut self, mut offset: usize) {
        // -> Result<(), String> {
        self.code.remove(offset);
        // for line in &mut self.lines {
        //     if pos < line.len() {
        //         line.remove(pos);
        //         return Ok(());
        //     }
        //     pos -= line.len();
        // }
        // Err(format!("Index out of bound"))
    }

    pub fn lines(&self) -> Vec<Vec<char>> {
        let mut lines = Vec::<Vec<char>>::new();
        let mut line = Vec::<char>::new();
        for &c in &self.code {
            if c == '\n' {
                line.push(' ');
                lines.push(line);
                line = Vec::<char>::new();
            } else {
                line.push(c);
            }
        }
        lines
    }

    pub fn code(&self) -> &Vec<char> {
        &self.code
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    fn bracklet_map(&mut self) -> Vec<usize> {
        let mut bracklet_map = vec![0usize; self.len()];
        let mut stack = Vec::new();
        for (i, char) in self.code.iter().enumerate() {
            match char {
                '[' => stack.push(i),
                ']' => {
                    let origin = stack.pop().expect(&format!(
                        "Closed bracklet at position {} without corresponding '['.",
                        i
                    ));
                    //     Some(indice) => indice,
                    //     None => {     return Err(format!(
                    //                        "Closed bracklet at position {} without corresponding '['.",
                    //                        i
                    //                    ));
                    //                }
                    // };
                    bracklet_map[i] = origin;
                    bracklet_map[origin] = i;
                }
                _ => {}
            }
        }
        // match stack.pop() {
        //     Some(indice) => {
        //         return Err(format!(
        //             "Opening bracklet found at position {} was never closed.",
        //             indice
        //         ));
        //     }
        //     None => {}
        // }
        // _ = stack.pop().expect(&format!(
        //     "Opening bracklet found at position {} was never closed.",
        //     indice
        // ));

        if stack.len() != 0 {
            for pos in stack {
                panic!("Opening bracklet was never closed.");
                // panic!(format!(
                //     "Opening bracklet found at position {} was never closed.",
                //     pos
                // ));
            }
        }
        bracklet_map
    }
}

#[derive(Clone)]
pub struct InterpreterState {
    // pub code: Vec<char>,
    // last_action: Option<char>,
    tape: Vec<u8>,
    pub ptr: usize,
    pub step: usize,
    output: Vec<u8>,
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
    state: InterpreterState,
    pub code: Code,
    // pub code: Vec<char>,
    bracklet_map: Vec<usize>,
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
    pub fn new(code: Code, tape_size: usize) -> Self {
        let mut interp = Self {
            state: InterpreterState::new(tape_size),
            bracklet_map: Vec::new(),
            code: code,
            waiting_for_input: false,
        };
        interp.build_bracklet_map();
        interp
    }

    pub fn reset(&mut self) {
        self.state = InterpreterState::new(self.state.tape.len());
        self.build_bracklet_map();
    }

    pub fn build_bracklet_map(&mut self) {
        self.bracklet_map = self.code.bracklet_map()
    }

    // pub fn code_insert(&mut self, pos: usize, c: char) {
    //     self.code.insert(pos, c);
    // }

    // pub fn code_del(&mut self, pos: usize) {
    //     self.code.remove(pos);
    // }

    // pub fn indent(&mut self, mut cursor_pos: usize) -> usize {
    //     let mut dept: usize = 0;
    //     let old_code = self.state.code.clone();
    //     self.state.code = Vec::new();
    //     // let mut line: Vec<char> = Vec::new();
    //     let mut offset: usize = 0;
    //     for (i, &c) in old_code.iter().enumerate() {
    //         if c == '[' {
    //             dept += 1;
    //             self.state.code.extend(vec![c, '\n']);
    //             self.state.code.extend(vec![' '; 2 * dept]);
    //             offset += 2 * (dept + 1)
    //         } else if c == '\n' {
    //             self.state.code.push('\n');
    //             self.state.code.extend(vec![' '; 2 * dept]);
    //         } else if c == ']' {
    //             self.state.code.push('\n');
    //             dept -= 1;
    //             self.state.code.extend(vec![' '; 2 * dept]);
    //             self.state.code.extend(vec![c, '\n']);
    //             self.state.code.extend(vec![' '; 2 * dept]);
    //         } else {
    //             self.state.code.push(c);
    //         }
    //     }
    //     cursor_pos
    // }

    pub fn exec_current_step(&mut self, entry: Option<u8>) -> Option<Effect> {
        if self.state.step == self.code.len() {
            return Some(Effect::End);
        }
        if self.waiting_for_input {
            self.state.tape[self.state.ptr] = entry.unwrap_or(0);
            // println!("Input received : {}", entry.unwrap_or(0));
            self.waiting_for_input = false;
            self.state.step += 1;
            return None;
        }

        match self.code.code()[self.state.step] {
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

    pub fn code(&self) -> &Code {
        &self.code
    }
}
