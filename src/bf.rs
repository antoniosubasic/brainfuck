use crate::memory::Memory;

pub struct Brainfuck {
    tape: Memory<i128>,
    instruction: Memory<char>,
    input: Vec<char>,
    loop_start: Vec<(usize, usize)>,
}

impl Brainfuck {
    pub fn new(code: &String, input: &String) -> Self {
        Self {
            tape: Memory::expanding(0),
            instruction: Memory::fixed(
                code.chars()
                    .filter(|c| vec!['<', '>', '+', '-', '.', ',', '[', ']'].contains(c))
                    .collect(),
            ),
            input: input.chars().collect(),
            loop_start: vec![],
        }
    }

    pub fn exec(&mut self) -> Result<(), String> {
        loop {
            let instruction = self.instruction.get_at_pointer()?;

            match instruction {
                '<' => {
                    self.tape.prev()?;
                }
                '>' => {
                    self.tape.next()?;
                }
                '+' | '-' => {
                    let is_add = *instruction == '+';
                    let operation = if is_add {
                        self.tape.get_at_pointer()?.checked_add(1)
                    } else {
                        self.tape.get_at_pointer()?.checked_sub(1)
                    };

                    match operation {
                        Some(value) => {
                            self.tape.set_at_pointer(value)?;
                        }
                        None => {
                            return Err(format!(
                                "value {} on tape at position: {}",
                                if is_add { "overflow" } else { "underflow" },
                                self.tape.get_pointer()
                            ));
                        }
                    }
                }
                '.' => {
                    let value = *self.tape.get_at_pointer()?;

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
                    self.tape.set_at_pointer(if self.input.is_empty() {
                        0
                    } else {
                        self.input.remove(0) as i128
                    })?;
                }
                '[' => {
                    self.loop_start
                        .push((self.instruction.get_pointer(), self.tape.get_pointer()));
                }
                ']' => match self.loop_start.last() {
                    Some(&(instruction_pointer, tape_pointer)) => {
                        if *self.tape.get_at(tape_pointer)? > 0 {
                            self.instruction.set_pointer(instruction_pointer);
                        } else {
                            self.loop_start.pop();
                        }
                    }
                    None => {
                        return Err(format!(
                            "unexpected ']' at {}",
                            self.instruction.get_pointer()
                        ));
                    }
                },
                _ => {
                    return Err(format!("invalid character: '{instruction}'"));
                }
            }

            match self.instruction.next() {
                Ok(_) => {}
                Err(e) => {
                    if e == "pointer overflow" {
                        break;
                    } else {
                        return Err(e.to_string());
                    }
                }
            }
        }

        Ok(())
    }
}
