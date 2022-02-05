use super::lexer::Lexer;
use super::token::{Token, Type};

pub struct BaseParser {
    lexer: Lexer,
    current: Option<Token>,
}

impl BaseParser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.get_next();
        Self {
            lexer,
            current: Some(current),
        }
    }

    fn advance(&mut self) {
        self.current = Some(self.lexer.get_next());
    }

    pub fn get_current(&mut self) -> Token {
        if self.current.is_none() {
            panic!("Error: Reached end of tokens (get current)");
        }

        return self.current.as_ref().unwrap().clone();
    }

    pub fn eat(&mut self, _type: Type) -> Result<Token, String> {
        if self.current.is_none() {
            panic!("Error: Reached end of tokens");
        }

        let current = self.current.as_ref().unwrap().clone();
        if current._type == _type {
            self.advance();
            return Ok(current);
        }

        return Err(String::from("Not matching type"));
    }

    pub fn eat_mult(&mut self, _types: &[Type]) -> Result<Token, String> {
        let mut last_err = Err("Nothing passed in".to_owned());
        for _type in _types {
            let token = self.eat(_type.to_owned());
            if token.is_ok() {
                return token;
            }

            last_err = token;
        }

        return last_err;
    }
}
