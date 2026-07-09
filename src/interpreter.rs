use std::fmt;

#[derive(Debug)]
pub struct Interpreter {
    pub raw_code: String,
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
            self.raw_code,
            &(self.tape)[..100],
        )
    }
}

impl Interpreter {
    pub fn new(raw_code: String, tape_size: usize) -> Self {
        Self {
            raw_code: raw_code,
            tape: vec![0u8; tape_size],
            ptr: 0,
            step: 0,
        }
    }
    pub fn exec_sbs() {}
}
