use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Add,
    Min,
    Mul,
    Div,
    Pow,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LAngleBracket,
    RAngleBracket,

    Dot,
    Comma,
    ExclaMark,
    Equal,
    BinAnd,
    BinOr,
    And,
    Or,
    Gt,
    Lt,
    NotEqual,
    Range,
    GtEqual,
    LtEqual,
    DEqual,
    AddEqual,
    MinEqual,
    MulEqual,
    DivEqual,

    Iden,
    ConstK,
    LetK,
    IfK,
    ElseK,
    FunkK,
    ReturnK,
    InK,
    PrintK,
    String,
    Num,

    EOL,
}

#[derive(Clone)]
pub struct Token {
    pub val: String,
    pub _type: Type,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Token")
            .field("val", &self.val)
            .field("_type", &self._type)
            .finish()
    }
}

impl Token {
    pub fn new(val: String, _type: Type) -> Self {
        Self { val, _type }
    }
}
