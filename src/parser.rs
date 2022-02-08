use crate::ast::node_box::NodeBox;
use crate::ast::node_kind::NodeKind;
use crate::ast::nodes::call_statement::NCallStatement;
use crate::ast::nodes::expression_statement::NExpressionStatement;
use crate::ast::nodes::identifier::NIdentifier;
use crate::ast::nodes::math_statement::NMathStatement;
use crate::ast::nodes::number::NNumber;
use crate::ast::nodes::program::NProgram;
use crate::ast::nodes::string::NString;
use crate::ast::nodes::variable_statement::NVariableStatement;

use super::base_parser::BaseParser;
use super::lexer::Lexer;
use super::token::Type;

pub struct Parser {
    base: BaseParser,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        return Self {
            base: BaseParser::new(lexer),
        };
    }

    pub fn gen_ast(lexer: Lexer) -> Box<NodeBox> {
        let mut parser = Parser::new(lexer);
        return parser.program().unwrap();
    }

    fn program(&mut self) -> Result<Box<NodeBox>, String> {
        let mut program = NProgram { childs: vec![] };
        while self.base.get_current()._type != Type::EOL {
            program.childs.push(self.expression_statement()?);
        }

        return Ok(NodeBox::new_box(Box::new(program), NodeKind::Program));
    }

    fn expression_statement(&mut self) -> Result<Box<NodeBox>, String> {
        let mut variable_statement = self.variable_statement()?;
        if variable_statement._type == NodeKind::Identifier {
            variable_statement = self.call_statement(variable_statement)?;
        }

        return Ok(NodeBox::new_box(
            Box::new(NExpressionStatement {
                content: variable_statement,
            }),
            NodeKind::ExpressionStatement,
        ));
    }

    fn variable_statement(&mut self) -> Result<Box<NodeBox>, String> {
        let delcare_token = self.base.eat_mult(&[Type::ConstK, Type::LetK]);
        let identifier = self.identifer()?;
        if self.base.eat(Type::Equal).is_err() {
            return Ok(identifier);
        }
        let math_statement = self.math_statement(false)?;

        return Ok(NodeBox::new_box(
            Box::new(NVariableStatement {
                declare_type: delcare_token.ok().and_then(|t| Some(t._type)),
                identifier,
                expression: math_statement,
            }),
            NodeKind::VariableStatement,
        ));
    }

    fn call_statement(&mut self, identifier: Box<NodeBox>) -> Result<Box<NodeBox>, String> {
        if self.base.eat(Type::LParen).is_err() {
            return Ok(identifier);
        }
        let mut params = vec![];
        while let Ok(basic_type) = self.basic_type() {
            params.push(basic_type);
            let _ = self.base.eat(Type::Comma);
        }
        self.base.eat(Type::RParen)?;
        return Ok(NodeBox::new_box(
            Box::new(NCallStatement {
                identifier,
                parameters: params,
            }),
            NodeKind::CallStatement,
        ));
    }

    fn math_statement(&mut self, as_paren: bool) -> Result<Box<NodeBox>, String> {
        if self.base.eat(Type::LParen).is_ok() {
            let math = self.math_statement(true);
            self.base.eat(Type::RParen)?;
            return math;
        }

        let left = self.basic_type()?;
        if let Ok(operator) = self
            .base
            .eat_mult(&[Type::Add, Type::Min, Type::Mul, Type::Div])
        {
            return Ok(NodeBox::new_box(
                Box::new(NMathStatement {
                    parenthese: as_paren,
                    left,
                    operator: operator.val,
                    right: self.math_statement(false)?,
                }),
                NodeKind::MathStatement,
            ));
        }

        return Ok(left);
    }

    fn basic_type(&mut self) -> Result<Box<NodeBox>, String> {
        if let Ok(identifier) = self.identifer() {
            let call_statement = self.call_statement(identifier)?;
            return Ok(call_statement);
        }
        if let Ok(string) = self.string() {
            return Ok(string);
        }
        return self.number();
    }

    fn identifer(&mut self) -> Result<Box<NodeBox>, String> {
        return Ok(NodeBox::new_box(
            Box::new(NIdentifier {
                name: self.base.eat(Type::Iden)?.val,
            }),
            NodeKind::Identifier,
        ));
    }

    fn string(&mut self) -> Result<Box<NodeBox>, String> {
        return Ok(NodeBox::new_box(
            Box::new(NString {
                val: self.base.eat(Type::String)?.val,
            }),
            NodeKind::String,
        ));
    }

    fn number(&mut self) -> Result<Box<NodeBox>, String> {
        return Ok(NodeBox::new_box(
            Box::new(NNumber {
                val: self.base.eat(Type::Num)?.val.parse().unwrap(),
            }),
            NodeKind::Number,
        ));
    }
}
