use super::token::{Token, Type};
use std::fmt::{Debug, Formatter, Result};

pub struct Lexer {
    pos: usize,
    chars: Vec<char>,
    current: Option<char>,
}

impl Debug for Lexer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Lexer")
            .field("pos", &self.pos)
            .field("chars", &self.chars)
            .finish()
    }
}

impl Lexer {
    pub fn new(exp: &str) -> Self {
        let chars: Vec<char> = exp.chars().collect();
        let current = chars[0];
        return Lexer {
            pos: 0,
            chars,
            current: Some(current),
        };
    }

    pub fn debug(mut self) {
        let mut current = self.get_next();
        while current._type != Type::EOL {
            println!("{:?}", current);
            current = self.get_next();
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.chars.len() {
            self.current = Some(self.chars[self.pos]);
        } else {
            self.current = None;
        }
    }

    fn peak(&mut self) -> Option<char> {
        if self.pos + 1 >= self.chars.len() {
            return None;
        }

        return Some(self.chars[self.pos + 1]);
    }

    fn is_to_skip(&self, c: char) -> bool {
        match c {
            ' ' => true,
            '\r' => true,
            '\n' => true,
            _ => false,
        }
    }

    fn handle_numeric(&mut self, c: char) -> Token {
        let mut num: String = c.to_string();
        self.advance();
        while let Some(current) = self.current {
            if !current.is_numeric() {
                break;
            }

            num += &current.to_string();
            self.advance();
        }

        return Token::new(num, Type::Num);
    }

    fn handle_double_special(&mut self, token: &Token) -> Option<Token> {
        let peak = self.peak();
        if peak.is_none() {
            return None;
        }

        let peak_c = peak.unwrap();
        let token_next = match token._type {
            Type::ExclaMark => match peak_c {
                '=' => Some(Type::NotEqual),
                _ => None,
            },
            Type::Equal => match peak_c {
                '=' => Some(Type::DEqual),
                _ => None,
            },
            Type::Gt => match peak_c {
                '=' => Some(Type::GtEqual),
                _ => None,
            },
            Type::Lt => match peak_c {
                '=' => Some(Type::LtEqual),
                _ => None,
            },
            Type::BinAnd => match peak_c {
                '&' => Some(Type::And),
                _ => None,
            },
            Type::BinOr => match peak_c {
                '|' => Some(Type::Or),
                _ => None,
            },
            Type::Dot => match peak_c {
                '.' => Some(Type::Range),
                _ => None,
            },
            Type::Add => match peak_c {
                '+' => Some(Type::Pow),
                '=' => Some(Type::AddEqual),
                _ => None,
            },
            Type::Min => match peak_c {
                '-' => Some(Type::Pow),
                '=' => Some(Type::MinEqual),
                _ => None,
            },
            Type::Mul => match peak_c {
                '*' => Some(Type::Pow),
                '=' => Some(Type::MulEqual),
                _ => None,
            },
            Type::Div => match peak_c {
                '=' => Some(Type::DivEqual),
                _ => None,
            },
            _ => None,
        };

        if token_next.is_some() {
            self.advance();
        }

        return token_next
            .and_then(|t| Some(Token::new(token.val.to_owned() + &peak_c.to_string(), t)));
    }

    fn handle_special(&mut self, c: char) -> Option<Token> {
        let mut _type = match c {
            '(' => Some(Type::LParen),
            ')' => Some(Type::RParen),
            '{' => Some(Type::LBracket),
            '}' => Some(Type::RBracket),
            '[' => Some(Type::LAngleBracket),
            ']' => Some(Type::RAngleBracket),
            '.' => Some(Type::Dot),
            ',' => Some(Type::Comma),
            '!' => Some(Type::ExclaMark),
            '=' => Some(Type::Equal),
            '>' => Some(Type::Gt),
            '<' => Some(Type::Lt),
            '&' => Some(Type::BinAnd),
            '|' => Some(Type::BinOr),
            '+' => Some(Type::Add),
            '-' => Some(Type::Min),
            '*' => Some(Type::Mul),
            '/' => Some(Type::Div),
            _ => None,
        };

        if _type.is_some() {
            let mut token = Some(Token::new(c.to_string(), _type.unwrap()));
            if let Some(token_double) = self.handle_double_special(&token.as_ref().unwrap()) {
                token = Some(token_double);
            }
            self.advance();
            return token;
        }

        return None;
    }

    fn handle_identifier(&mut self, c: char) -> Token {
        let mut iden = c.to_string();
        self.advance();
        while let Some(current) = self.current {
            if !current.is_ascii_alphanumeric() && current != '_' {
                break;
            }

            iden += &current.to_string();
            self.advance();
        }

        return Token::new(iden, Type::Iden);
    }

    fn handle_keyword(&mut self, iden: &str) -> Option<Token> {
        let _type = match iden.to_lowercase().as_str() {
            "const" => Some(Type::ConstK),
            "let" => Some(Type::LetK),
            "if" => Some(Type::IfK),
            "else" => Some(Type::ElseK),
            "funk" => Some(Type::FunkK),
            "return" => Some(Type::ReturnK),
            "in" => Some(Type::InK),
            "print" => Some(Type::PrintK),
            _ => None,
        };

        return _type.and_then(|_t| Some(Token::new(iden.to_owned(), _t)));
    }

    fn handle_string_escape(&mut self) -> char {
        if self.current.is_none() {
            panic!("Error: Nothing to escape");
        }

        let espace_char = self.current.unwrap();
        match espace_char {
            'r' => '\r',
            'n' => '\n',
            't' => '\t',
            _ => espace_char,
        }
    }

    fn handle_string(&mut self) -> Token {
        let mut str = String::new();
        self.advance();
        while let Some(current) = self.current {
            if current == '"' {
                break;
            }

            if current == '\\' {
                self.advance();
                str += &self.handle_string_escape().to_string();
            } else {
                str += &current.to_string();
            }

            self.advance();
        }

        if (self.current.is_some() && self.current.unwrap() != '"') || self.current.is_none() {
            panic!("Error: Unexpected end of input -> ({:?})", self.current);
        }

        self.advance();
        return Token::new(str, Type::String);
    }

    pub fn get_next(&mut self) -> Token {
        while let Some(current) = self.current {
            // Char stuff
            if self.is_to_skip(current) {
                self.advance();
                continue;
            } else if current.is_numeric() {
                return self.handle_numeric(current);
            } else if let Some(token) = self.handle_special(current) {
                return token;
            }

            // Text stuff
            if current.is_alphabetic() {
                let token = self.handle_identifier(current);
                if let Some(t) = self.handle_keyword(&token.val) {
                    return t;
                }
                return token;
            } else if current == '"' {
                return self.handle_string();
            }

            panic!("Error: Unexpected start of input -> ({}) ", current);
        }

        return Token::new(String::from(""), Type::EOL);
    }
}
