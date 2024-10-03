pub struct Memory<T> {
    default_value: Option<T>,
    data: Vec<T>,
    pointer: usize,
}

impl<T> Memory<T> {
    pub fn expanding(default_value: T) -> Self
    where
        T: Clone,
    {
        Self {
            default_value: Some(default_value.clone()),
            data: vec![default_value; 1],
            pointer: 0,
        }
    }

    pub fn fixed(data: Vec<T>) -> Self
    where
        T: Clone,
    {
        Self {
            default_value: None,
            data,
            pointer: 0,
        }
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    pub fn set_pointer(&mut self, pointer: usize) {
        self.pointer = pointer;
    }

    pub fn move_pointer(&mut self, offset: isize) -> Result<(), &str>
    where
        T: Clone,
    {
        let new_pointer = self.pointer as isize + offset;

        if new_pointer < 0 {
            if let Some(default_value) = &self.default_value {
                self.pointer = 0;
                self.data
                    .splice(0..0, vec![default_value.clone(); -new_pointer as usize]);
            } else {
                return Err("pointer underflow");
            }
        } else if new_pointer >= self.data.len() as isize {
            if let Some(default_value) = &self.default_value {
                self.data.extend(vec![
                    default_value.clone();
                    new_pointer as usize - self.data.len() + 1
                ]);
                self.pointer = new_pointer as usize;
            } else {
                return Err("pointer overflow");
            }
        } else {
            self.pointer = new_pointer as usize;
        }

        Ok(())
    }

    pub fn next(&mut self) -> Result<(), &str>
    where
        T: Clone,
    {
        self.move_pointer(1)
    }

    pub fn prev(&mut self) -> Result<(), &str>
    where
        T: Clone,
    {
        self.move_pointer(-1)
    }

    pub fn get_at(&self, index: usize) -> Result<&T, String> {
        if index < self.data.len() {
            Ok(&self.data[index])
        } else {
            Err(format!(
                "index ({}) out of bounds (0..{})",
                index,
                self.data.len()
            ))
        }
    }

    pub fn set_at(&mut self, index: usize, value: T) -> Result<(), String>
    where
        T: Clone,
    {
        if index < self.data.len() {
            self.data[index] = value;
            Ok(())
        } else {
            Err(format!(
                "index ({}) out of bounds (0..{})",
                index,
                self.data.len()
            ))
        }
    }

    pub fn get_at_pointer(&self) -> Result<&T, String> {
        self.get_at(self.pointer)
    }

    pub fn set_at_pointer(&mut self, value: T) -> Result<(), String>
    where
        T: Clone,
    {
        self.set_at(self.pointer, value)
    }
}
