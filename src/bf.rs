pub struct Brainfuck {
    tape: Vec<i128>,
    ip: u128,
    code: String,
    input: Option<Vec<char>>,
}

impl Brainfuck {
    pub fn new(code: String, input: Option<&String>) -> Self {
        Self {
            tape: vec![],
            ip: 0,
            code,
            input: input.map(|s| s.chars().collect()),
        }
    }

    pub fn exec(&self) -> Result<(), &str> {
        Ok(())
    }
}
