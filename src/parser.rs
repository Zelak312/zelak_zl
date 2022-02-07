use super::baseParser::BaseParser;
use super::lexer::Lexer;
use super::token::Type;

use crate::nodes::expression_statement::NExpressionStatement;
use crate::nodes::identifier::NIdentifier;
use crate::nodes::number::NNumber;
use crate::nodes::program::NProgram;
use crate::nodes::variable_statement::NVariableStatement;

pub struct Parser {
    base: BaseParser,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        return Self {
            base: BaseParser::new(lexer),
        };
    }

    pub fn gen_ast(lexer: Lexer) -> Box<NProgram> {
        let mut parser = Parser::new(lexer);
        return parser.program().unwrap();
    }

    fn program(&mut self) -> Result<Box<NProgram>, String> {
        let mut program = NProgram { childs: vec![] };
        while self.base.get_current()._type != Type::EOL {
            program.childs.push(self.variable_statement()?);
        }

        return Ok(Box::new(program));
    }

    fn variable_statement(&mut self) -> Result<Box<NVariableStatement>, String> {
        let delcare_token = self.base.eat_mult(&[Type::ConstK, Type::LetK]);
        let identifier = self.identifer()?;
        self.base.eat(Type::Equal)?;
        let expression_statement = self.expression_statement()?;

        return Ok(Box::new(NVariableStatement {
            declare_type: delcare_token.ok().and_then(|t| Some(t._type)),
            identifier,
            expression: expression_statement,
        }));
    }

    fn expression_statement(&mut self) -> Result<Box<NExpressionStatement>, String> {
        let number = self.number()?;
        return Ok(Box::new(NExpressionStatement { content: number }));
    }

    fn identifer(&mut self) -> Result<Box<NIdentifier>, String> {
        return Ok(Box::new(NIdentifier {
            name: self.base.eat(Type::Iden)?.val,
        }));
    }

    fn number(&mut self) -> Result<Box<NNumber>, String> {
        return Ok(Box::new(NNumber {
            val: self.base.eat(Type::Num)?.val.parse().unwrap(),
        }));
    }
}
