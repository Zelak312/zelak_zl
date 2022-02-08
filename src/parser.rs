use crate::ast::node_box::NodeBox;
use crate::ast::node_kind::NodeKind;
use crate::ast::nodes::expression_statement::NExpressionStatement;
use crate::ast::nodes::identifier::NIdentifier;
use crate::ast::nodes::math_statement::NMathStatement;
use crate::ast::nodes::number::NNumber;
use crate::ast::nodes::program::NProgram;
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
            program.childs.push(self.variable_statement()?);
        }

        return Ok(NodeBox::new_box(Box::new(program), NodeKind::Program));
    }

    fn variable_statement(&mut self) -> Result<Box<NodeBox>, String> {
        let delcare_token = self.base.eat_mult(&[Type::ConstK, Type::LetK]);
        let identifier = self.identifer()?;
        self.base.eat(Type::Equal)?;
        let expression_statement = self.expression_statement()?;

        return Ok(NodeBox::new_box(
            Box::new(NVariableStatement {
                declare_type: delcare_token.ok().and_then(|t| Some(t._type)),
                identifier,
                expression: expression_statement,
            }),
            NodeKind::VariableStatement,
        ));
    }

    fn expression_statement(&mut self) -> Result<Box<NodeBox>, String> {
        let math = self.math_statement(false)?;
        return Ok(NodeBox::new_box(
            Box::new(NExpressionStatement { content: math }),
            NodeKind::ExpressionStatement,
        ));
    }

    fn math_statement(&mut self, as_paren: bool) -> Result<Box<NodeBox>, String> {
        if self.base.eat(Type::LParen).is_ok() {
            let math = self.math_statement(true);
            self.base.eat(Type::RParen)?;
            return math;
        }

        let left = self.number()?;
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

    fn identifer(&mut self) -> Result<Box<NodeBox>, String> {
        return Ok(NodeBox::new_box(
            Box::new(NIdentifier {
                name: self.base.eat(Type::Iden)?.val,
            }),
            NodeKind::Identifier,
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
