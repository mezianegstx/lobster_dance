use std::fmt;

#[derive(Debug)]
pub struct Interpreter {
    pub code: Vec<char>,
    pub bracklet_map: Vec<usize>,
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
        let mut interp = Self {
            code: raw_code.chars().collect(),
            bracklet_map: vec![0usize; raw_code.len()],
            tape: vec![0u8; tape_size],
            ptr: 0,
            step: 0,
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

    pub fn exec_current_step(&mut self) {
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
            _ => {}
        };
        self.step += 1;
    }

    // pub fn exec(&mut self, options: InterpOptions) {
    //     println!("{:?}", self.bracklet_map);
    //     while self.step < self.code.len() {
    //         self.exec_sbs();
    //         thread::sleep(Duration::from_millis(options.delay_ms));
    //         if options.verbose {
    //             println!("{} {:?}", self.code[self.step - 1], &(self.tape)[..50])
    //         }
    //     }
    // }
    //
    pub fn tape(&self) -> &[u8] {
        &self.tape
    }
}
