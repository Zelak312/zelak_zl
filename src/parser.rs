use std::any::Any;

use super::baseParser::BaseParser;
use super::lexer::Lexer;
use super::token::Type;

use crate::zl_nodes::zassignment::ZAssignment;
use crate::zl_nodes::zexpr::ZExpr;
use crate::zl_nodes::zfunction_call::ZFunction_call;
use crate::zl_nodes::ziden::ZIden;
use crate::zl_nodes::znumber::ZNumber;
use crate::zl_nodes::zstring::ZString;

pub struct Parser {
    base: BaseParser,
}

/*
expr = (FuncCall|Statement)*
FuncCall = ([PrintK]|[Iden])+[LParen]+AllType+[RParen] TODO: support more args
Statement = Declare+[Equal]+AllType
Declare = ([ConstK]|[LetK])?+[Iden]
AllType = ([Iden]|[String]|[Num])
*/

impl Parser {
    fn new(lexer: Lexer) -> Self {
        return Self {
            base: BaseParser::new(lexer),
        };
    }

    pub fn gen_ast(lexer: Lexer) -> Box<ZExpr> {
        let mut parser = Parser::new(lexer);
        return parser.expr().unwrap();
    }

    fn expr(&mut self) -> Result<Box<ZExpr>, String> {
        let mut current = self.base.get_current();
        let mut expr = ZExpr { childs: vec![] };
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

    fn func_call(&mut self) -> Result<Box<ZFunction_call>, String> {
        let func_name = match self.base.get_current()._type {
            Type::PrintK => {
                self.base.eat(Type::PrintK).unwrap();
                Some("print".to_owned())
            }
            Type::Iden => Some(self.base.eat(Type::Iden).unwrap().val),
            _ => None,
        }
        .ok_or("Couldn't find print or iden")?;

        self.base.eat(Type::LParen)?;
        let all_type = self.all_type()?;
        self.base.eat(Type::RParen)?;

        return Ok(Box::new(ZFunction_call {
            func_name,
            parameters: { vec![all_type] },
        }));
    }

    fn statement(&mut self) -> Result<Box<ZAssignment>, String> {
        let declare = self.declare()?;
        self.base.eat(Type::Equal)?;
        let all_type = self.all_type()?;

        return Ok(Box::new(ZAssignment {
            declare_type: declare.0,
            iden: declare.1,
            content: all_type,
        }));
    }

    fn declare(&mut self) -> Result<Box<(Option<Type>, String)>, String> {
        let token = match self.base.get_current()._type {
            Type::ConstK => Some(self.base.eat(Type::ConstK).unwrap()._type),
            Type::LetK => Some(self.base.eat(Type::LetK).unwrap()._type),
            _ => None,
        };

        let iden = self.base.eat(Type::Iden)?;
        return Ok(Box::new((token, iden.val)));
    }

    fn all_type(&mut self) -> Result<Box<dyn Any>, String> {
        let _type: Option<Box<dyn Any>> = match self.base.get_current()._type {
            Type::Iden => Some(Box::new(ZIden {
                name: self.base.eat(Type::Iden).unwrap().val,
            })),
            Type::String => Some(Box::new(ZString {
                val: self.base.eat(Type::String).unwrap().val,
            })),
            Type::Num => Some(Box::new(ZNumber {
                val: self.base.eat(Type::Num).unwrap().val.parse().unwrap(),
            })),
            _ => None,
        };

        if _type.is_none() {
            return Err("Didn't find type in all_type".to_string());
        }

        return Ok(_type.unwrap());
    }
}
