use std::any::Any;

use super::baseParser::BaseParser;
use super::lexer::Lexer;
use super::token::Type;

use crate::nodes::assignment::Assignment;
use crate::nodes::bin_op::BinOp;
use crate::nodes::expr::Expr;
use crate::nodes::function_call::FunctionCall;
use crate::nodes::iden::Iden;
use crate::nodes::number::Number;
use crate::nodes::string::NString;
use crate::token::Token;

pub struct Parser {
    base: BaseParser,
}

/*
Expr = (FuncCall|Statement)*
FuncCall = ([PrintK]|[Iden])+Parameters
Parameters = [LParen]+MathOrString([Comma]+MathOrString)*+[RParen]
Statement = Declare+[Equal]+MathOrString
Declare = ([ConstK]|[LetK])?+[Iden]
MathType = ([Iden]|[Num])

BinOp = ([Add]|[Min]|[Multi]|[Div])
MathOrString = (MathExpr|[String])
MathExpr = [LParen]?+MathType+(BinOp+MathExpr)*[RParen]?
*/

impl Parser {
    fn new(lexer: Lexer) -> Self {
        return Self {
            base: BaseParser::new(lexer),
        };
    }

    pub fn gen_ast(lexer: Lexer) -> Box<Expr> {
        let mut parser = Parser::new(lexer);
        return parser.expr().unwrap();
    }

    fn expr(&mut self) -> Result<Box<Expr>, String> {
        let mut current = self.base.get_current();
        let mut expr = Expr { childs: vec![] };
        while current._type != Type::EOL {
            let func_call = self.func_call();
            if func_call.is_ok() {
                expr.childs.push(func_call.unwrap());
                current = self.base.get_current();
                continue;
            }

            let statement = self.statement()?;
            expr.childs.push(statement);
            current = self.base.get_current();
        }

        return Ok(Box::new(expr));
    }

    fn func_call(&mut self) -> Result<Box<FunctionCall>, String> {
        let func_token = self
            .base
            .eat_mult(&[Type::PrintK, Type::Iden])
            .or(Err("Couldn't find print or iden"))?;

        return Ok(Box::new(FunctionCall {
            func_name: func_token.val,
            parameters: { self.parameters()? },
        }));
    }

    fn parameters(&mut self) -> Result<Vec<Box<dyn Any>>, String> {
        self.base.eat(Type::LParen)?;
        let mut params = vec![];
        params.push(self.math_or_string()?);
        while let Ok(_) = self.base.eat(Type::Comma) {
            params.push(self.math_or_string()?);
        }
        self.base.eat(Type::RParen)?;

        return Ok(params);
    }

    fn statement(&mut self) -> Result<Box<Assignment>, String> {
        let declare = self.declare()?;
        self.base.eat(Type::Equal)?;
        let all_type = self.math_or_string()?;

        return Ok(Box::new(Assignment {
            declare_type: declare.0,
            iden: declare.1,
            content: all_type,
        }));
    }

    fn declare(&mut self) -> Result<Box<(Option<Type>, String)>, String> {
        let token = self
            .base
            .eat_mult(&[Type::ConstK, Type::LetK])
            .ok()
            .and_then(|t| Some(t._type));

        let iden = self.base.eat(Type::Iden)?;
        return Ok(Box::new((token, iden.val)));
    }

    fn math_type(&mut self) -> Result<Box<dyn Any>, String> {
        let token = self.base.eat_mult(&[Type::Iden, Type::Num])?;
        let node: Option<Box<dyn Any>> = match token._type {
            Type::Iden => Some(Box::new(Iden { name: token.val })),
            Type::Num => Some(Box::new(Number {
                val: token.val.parse().unwrap(),
            })),
            _ => None,
        };

        if node.is_none() {
            return Err("Didn't find type in all_type".to_string());
        }

        return Ok(node.unwrap());
    }

    fn bin_op(&mut self) -> Result<Token, String> {
        return self
            .base
            .eat_mult(&[Type::Add, Type::Min, Type::Mul, Type::Div]);
    }

    fn math_or_string(&mut self) -> Result<Box<dyn Any>, String> {
        let math_expr = self.math_expr();
        if math_expr.is_ok() {
            return math_expr;
        }

        return Ok(Box::new(NString {
            val: self.base.eat(Type::String).unwrap().val,
        }));
    }

    fn math_expr(&mut self) -> Result<Box<dyn Any>, String> {
        let lparen = self.base.eat(Type::LParen);
        let math_type = self.math_type()?;
        if let Ok(bin_op) = self.bin_op() {
            let right = self.math_expr()?;
            if lparen.is_ok() {
                self.base.eat(Type::RParen)?;
            }

            return Ok(Box::new(BinOp {
                op: bin_op.val,
                parenthese: lparen.is_ok(),
                left: math_type,
                right,
            }));
        } else {
            return Ok(math_type);
        }
    }
}
