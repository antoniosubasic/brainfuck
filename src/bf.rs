use crate::memory::Memory;

pub struct Brainfuck {
    tape: Memory<u8>,
    instruction: Memory<char>,
    input: Vec<char>,
    loop_start: Vec<usize>,
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
            let instruction = *self.instruction.get_at_pointer()?;

            match instruction {
                '<' => {
                    self.tape.prev()?;
                }
                '>' => {
                    self.tape.next()?;
                }
                '+' | '-' => {
                    let value = *self.tape.get_at_pointer()?;

                    self.tape.set_at_pointer(if instruction == '+' {
                        value + 1
                    } else {
                        value - 1
                    })?;
                }
                '.' => {
                    let value = *self.tape.get_at_pointer()?;

                    if let Some(c) = std::char::from_u32(value as u32) {
                        print!("{c}");
                    } else {
                        return Err(format!("invalid unicode value: {}", value));
                    }
                }
                ',' => {
                    self.tape.set_at_pointer(if self.input.is_empty() {
                        0
                    } else {
                        self.input.remove(0) as u8
                    })?;
                }
                '[' => {
                    self.loop_start.push(self.instruction.get_pointer());
                }
                ']' => match self.loop_start.last() {
                    Some(&instruction_pointer) => {
                        if *self.tape.get_at_pointer()? > 0 {
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
