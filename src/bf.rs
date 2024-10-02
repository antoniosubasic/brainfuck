use std::collections::VecDeque;

struct Tape {
    data: Vec<i128>,
    pointer: usize,
}

impl Tape {
    fn new() -> Self {
        Self {
            data: vec![0],
            pointer: 0,
        }
    }
}

pub struct Brainfuck {
    tape: Tape,
    ip: usize,
    code: Vec<char>,
    input: Vec<char>,
    loop_start: VecDeque<(usize, usize)>,
}

impl Brainfuck {
    pub fn new(code: &String, input: &String) -> Self {
        Self {
            tape: Tape::new(),
            ip: 0,
            code: code
                .chars()
                .filter(|c| vec!['<', '>', '+', '-', '.', ',', '[', ']'].contains(c))
                .collect(),
            input: input.chars().collect(),
            loop_start: VecDeque::new(),
        }
    }

    pub fn exec(&mut self) -> Result<(), String> {
        while self.ip < self.code.len() {
            let instruction = self.code[self.ip];

            match instruction {
                '<' => {
                    if self.tape.pointer == 0 {
                        self.tape.data.insert(0, 0);
                    } else {
                        self.tape.pointer -= 1;
                    }
                }
                '>' => {
                    if self.tape.pointer == self.tape.data.len() - 1 {
                        self.tape.data.push(0);
                    }

                    self.tape.pointer += 1;
                }
                '+' | '-' => {
                    let is_add = instruction == '+';
                    let operation = if is_add {
                        self.tape.data[self.tape.pointer].checked_add(1)
                    } else {
                        self.tape.data[self.tape.pointer].checked_sub(1)
                    };

                    match operation {
                        Some(value) => self.tape.data[self.tape.pointer] = value,
                        None => {
                            return Err(format!(
                                "value {} on tape at position: {}",
                                if is_add { "overflow" } else { "underflow" },
                                self.tape.pointer
                            ));
                        }
                    }
                }
                '.' => {
                    let value = self.tape.data[self.tape.pointer];

                    if value >= u32::MIN as i128 && value <= u32::MAX as i128 {
                        if let Some(c) = std::char::from_u32(value as u32) {
                            print!("{c}");
                        } else {
                            return Err(format!("invalid unicode value: {}", value));
                        }
                    } else {
                        return Err(format!("invalid unicode value: {}", value));
                    }
                }
                ',' => {
                    self.tape.data[self.tape.pointer] = if self.input.is_empty() {
                        0
                    } else {
                        self.input.remove(0) as i128
                    };
                }
                '[' => {
                    self.loop_start.push_back((self.ip, self.tape.pointer));
                }
                ']' => match self.loop_start.back() {
                    Some(&(ip, data_pointer)) => {
                        if self.tape.data[data_pointer] > 0 {
                            self.ip = ip;
                        } else {
                            self.loop_start.pop_back();
                        }
                    }
                    None => {
                        return Err("reached loop-end without loop-start".to_string());
                    }
                },
                _ => {
                    return Err(format!("invalid character: '{instruction}'"));
                }
            }

            self.ip += 1;
        }

        Ok(())
    }
}
