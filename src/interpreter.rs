use std::fmt;

#[derive(Debug)]
pub struct Interpreter {
    pub code: Vec<char>,
    pub tape: Vec<u8>,
    pub ptr: usize,
    pub step: usize,
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

impl Interpreter {
    pub fn new(raw_code: String, tape_size: usize) -> Self {
        Self {
            code: raw_code.chars().collect(),
            tape: vec![0u8; tape_size],
            ptr: 0,
            step: 0,
        }
    }
    pub fn exec_sbs(&mut self) {
        match self.code[self.step] {
            '>' => self.ptr += 1,
            '<' => self.ptr -= 1,
            '+' => self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(1),
            '-' => self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(1),
            _ => {}
        }
    }
    pub fn exec(&mut self) {
        while self.step < self.code.len() {
            self.exec_sbs();
            self.step += 1
        }
    }
}
