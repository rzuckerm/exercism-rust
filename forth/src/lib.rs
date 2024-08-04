use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
type ValueResult = std::result::Result<Value, Error>;
type VectorResult = std::result::Result<Vec<Value>, Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    user_defs: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_lowercase();
        let mut input_iter = input.split_whitespace();
        while let Some(word) = input_iter.next() {
            match word {
                ":" => {
                    let name = input_iter.next().ok_or(Error::InvalidWord)?;
                    name.parse::<Value>().err().ok_or(Error::InvalidWord)?;
                    let mut instructions: String = String::from(" ");
                    loop {
                        match input_iter.next() {
                            Some(";") => break,
                            Some(instruction) => instructions += &format!("{instruction} "),
                            None => Err(Error::InvalidWord)?,
                        }
                    }

                    let search = format!(" {name} ");
                    if let Some(old_instructions) =
                        self.user_defs.insert(name.to_string(), instructions)
                    {
                        for value in self.user_defs.values_mut() {
                            *value = value.replace(&search, &old_instructions);
                        }
                    }
                }
                _ if self.user_defs.contains_key(word) => {
                    self.eval(&self.user_defs[word].clone())?
                }
                "+" => self.binary_op(|a, b| Ok(vec![a + b]))?,
                "-" => self.binary_op(|a, b| Ok(vec![a - b]))?,
                "*" => self.binary_op(|a, b| Ok(vec![a * b]))?,
                "/" => self.binary_op(|a, b| match a.checked_div(b) {
                    Some(x) => Ok(vec![x]),
                    None => Err(Error::DivisionByZero),
                })?,
                "dup" => self.unary_op(|a| Ok(vec![a, a]))?,
                "drop" => self.unary_op(|_| Ok(vec![]))?,
                "swap" => self.binary_op(|a, b| Ok(vec![b, a]))?,
                "over" => self.binary_op(|a, b| Ok(vec![a, b, a]))?,
                _ => {
                    self.stack
                        .push(word.parse::<Value>().or(Err(Error::UnknownWord))?);
                }
            }
        }

        Ok(())
    }

    fn binary_op(&mut self, f: fn(Value, Value) -> VectorResult) -> Result {
        let (rhs, lhs) = (self.pop()?, self.pop()?);
        Ok(self.stack.extend(f(lhs, rhs)?))
    }

    fn unary_op(&mut self, f: fn(Value) -> VectorResult) -> Result {
        let x = self.pop()?;
        Ok(self.stack.extend(f(x)?))
    }

    fn pop(&mut self) -> ValueResult {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }
}
