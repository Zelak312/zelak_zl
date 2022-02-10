use crate::ast::node_box::NodeBox;
use crate::ast::node_kind::NodeKind;
use crate::ast::nodes::array::NArray;
use crate::ast::nodes::boolean::NBoolean;
use crate::ast::nodes::call_statement::NCallStatement;
use crate::ast::nodes::condition::NCondition;
use crate::ast::nodes::condition_statement::NConditionStatement;
use crate::ast::nodes::expression_statement::NExpressionStatement;
use crate::ast::nodes::for_statement::NForStatement;
use crate::ast::nodes::function_definition::NFunctionDefinition;
use crate::ast::nodes::function_return::NFunctionReturn;
use crate::ast::nodes::identifier::NIdentifier;
use crate::ast::nodes::if_statement::NIfStatement;
use crate::ast::nodes::math_statement::NMathStatement;
use crate::ast::nodes::number::NNumber;
use crate::ast::nodes::parenthese_statement::NParentheseStatement;
use crate::ast::nodes::program::NProgram;
use crate::ast::nodes::string::NString;
use crate::ast::nodes::string_concat::NStringConcat;
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

    fn function_definition(&mut self) -> Result<Box<NodeBox>, String> {
        self.base.eat(Type::FunkK)?;
        let identifier = self.identifer()?;
        self.base.eat(Type::LParen)?;
        let arguments = self.function_parameters(true)?;
        self.base.eat(Type::RParen)?;
        self.base.eat(Type::LBracket)?;
        let mut expressions = vec![];
        while let Ok(expression) = self.expression_statement() {
            expressions.push(expression);
        }
        self.base.eat(Type::RBracket)?;

        return Ok(NodeBox::new_box(
            Box::new(NFunctionDefinition {
                identifier,
                arguments,
                expressions,
            }),
            NodeKind::FunctionDefinition,
        ));
    }

    fn expression_statement(&mut self) -> Result<Box<NodeBox>, String> {
        if let Ok(if_statement) = self.if_statement() {
            return Ok(NodeBox::new_box(
                Box::new(NExpressionStatement {
                    content: if_statement,
                }),
                NodeKind::ExpressionStatement,
            ));
        }
        if let Ok(for_statement) = self.for_statement() {
            return Ok(NodeBox::new_box(
                Box::new(NExpressionStatement {
                    content: for_statement,
                }),
                NodeKind::ExpressionStatement,
            ));
        }
        if let Ok(function_definition) = self.function_definition() {
            return Ok(NodeBox::new_box(
                Box::new(NExpressionStatement {
                    content: function_definition,
                }),
                NodeKind::ExpressionStatement,
            ));
        }
        if let Ok(function_return) = self.function_return() {
            return Ok(NodeBox::new_box(
                Box::new(NExpressionStatement {
                    content: function_return,
                }),
                NodeKind::ExpressionStatement,
            ));
        }
        let variable_statement = self.variable_statement();
        let content = self.call_statement(variable_statement)?;

        return Ok(NodeBox::new_box(
            Box::new(NExpressionStatement { content }),
            NodeKind::ExpressionStatement,
        ));
    }

    fn function_return(&mut self) -> Result<Box<NodeBox>, String> {
        self.base.eat(Type::ReturnK)?;
        return Ok(NodeBox::new_box(
            Box::new(NFunctionReturn {
                content: self.condition_statement_or_basic_type()?,
            }),
            NodeKind::FunctionReturn,
        ));
    }

    fn if_statement(&mut self) -> Result<Box<NodeBox>, String> {
        self.base.eat(Type::IfK)?;
        self.base.eat(Type::LParen)?;
        let condition = self.condition_statement()?;
        self.base.eat(Type::RParen)?;
        self.base.eat(Type::LBracket)?;
        let mut expressions = vec![];
        while let Ok(expression) = self.expression_statement() {
            expressions.push(expression);
        }

        self.base.eat(Type::RBracket)?;
        return Ok(NodeBox::new_box(
            Box::new(NIfStatement {
                condition,
                expressions,
            }),
            NodeKind::IfStatement,
        ));
    }

    fn for_statement(&mut self) -> Result<Box<NodeBox>, String> {
        self.base.eat(Type::ForK)?;
        self.base.eat(Type::LParen)?;
        let start = self.variable_statement();
        self.base.eat(Type::SemiCol)?;
        let condition = self.condition()?;
        self.base.eat(Type::SemiCol)?;
        let step = self.variable_statement();
        self.base.eat(Type::RParen)?;
        self.base.eat(Type::LBracket)?;

        let mut expressions = vec![];
        while let Ok(expression) = self.expression_statement() {
            expressions.push(expression);
        }
        self.base.eat(Type::RBracket)?;
        return Ok(NodeBox::new_box(
            Box::new(NForStatement {
                start: start.ok(),
                condition,
                step: step.ok(),
                expressions,
            }),
            NodeKind::ForStatement,
        ));
    }

    fn variable_statement(&mut self) -> Result<Box<NodeBox>, String> {
        let delcare_token = self.base.eat_mult(&[Type::ConstK, Type::LetK]);
        let identifier = self.identifer()?;
        if self.base.eat(Type::Equal).is_err() {
            return Ok(identifier);
        }
        let condition_statement = self.condition_statement_or_basic_type()?;

        return Ok(NodeBox::new_box(
            Box::new(NVariableStatement {
                declare_type: delcare_token.ok().and_then(|t| Some(t._type)),
                identifier,
                expression: condition_statement,
            }),
            NodeKind::VariableStatement,
        ));
    }

    fn call_statement(
        &mut self,
        identifier: Result<Box<NodeBox>, String>,
    ) -> Result<Box<NodeBox>, String> {
        let function_name;
        if identifier.is_err() {
            function_name = self.function_reserved_identifier()?;
        } else if identifier.as_ref().unwrap()._type == NodeKind::Identifier {
            function_name = identifier.unwrap();
        } else {
            return identifier;
        }

        if self.base.eat(Type::LParen).is_err() {
            return Ok(function_name);
        }
        let params = self.function_parameters(false)?;
        self.base.eat(Type::RParen)?;
        return Ok(NodeBox::new_box(
            Box::new(NCallStatement {
                identifier: function_name,
                parameters: params,
            }),
            NodeKind::CallStatement,
        ));
    }

    fn function_parameters(&mut self, only_iden: bool) -> Result<Vec<Box<NodeBox>>, String> {
        let mut params = vec![];
        let current = match only_iden {
            true => self.identifer(),
            false => self.condition_statement_or_basic_type(),
        };

        if current.is_ok() {
            params.push(current.unwrap());
            while let Ok(_) = self.base.eat(Type::Comma) {
                let param = match only_iden {
                    true => self.identifer(),
                    false => self.condition_statement_or_basic_type(),
                }?;
                params.push(param);
            }
        }

        return Ok(params);
    }

    fn function_reserved_identifier(&mut self) -> Result<Box<NodeBox>, String> {
        return Ok(NodeBox::new_box(
            Box::new(NIdentifier {
                name: self.base.eat_mult(&[Type::PrintK])?.val,
            }),
            NodeKind::Identifier,
        ));
    }

    fn condition_statement_or_basic_type(&mut self) -> Result<Box<NodeBox>, String> {
        if let Ok(condition_statement) = self.condition_statement() {
            return Ok(condition_statement);
        }

        return self.basic_type();
    }

    fn condition_statement(&mut self) -> Result<Box<NodeBox>, String> {
        let left = self.condition()?;
        if let Ok(operator) = self.base.eat_mult(&[Type::And, Type::Or]) {
            return Ok(NodeBox::new_box(
                Box::new(NConditionStatement {
                    operator: operator.val,
                    left,
                    right: self.condition_statement()?,
                }),
                NodeKind::ConditionStatement,
            ));
        }

        return Ok(left);
    }

    fn condition(&mut self) -> Result<Box<NodeBox>, String> {
        let mut left = self.string_concat();
        if left.is_err() {
            left = self.boolean();
        }
        let left_val = left?;
        if let Ok(operator) = self.base.eat_mult(&[
            Type::Gt,
            Type::Lt,
            Type::NotEqual,
            Type::GtEqual,
            Type::LtEqual,
            Type::DEqual,
        ]) {
            return Ok(NodeBox::new_box(
                Box::new(NCondition {
                    operator: operator.val,
                    left: left_val,
                    right: self.condition()?,
                }),
                NodeKind::Condition,
            ));
        }

        return Ok(left_val);
    }

    fn math_statement(&mut self) -> Result<Box<NodeBox>, String> {
        let left = self.parenthese_statement()?;
        if let Ok(operator) = self
            .base
            .eat_mult(&[Type::Add, Type::Min, Type::Mul, Type::Div])
        {
            return Ok(NodeBox::new_box(
                Box::new(NMathStatement {
                    left,
                    operator: operator.val,
                    right: self.math_statement()?,
                }),
                NodeKind::MathStatement,
            ));
        }

        return Ok(left);
    }

    fn string_concat(&mut self) -> Result<Box<NodeBox>, String> {
        let left = self.string_concat_type()?;
        if let Ok(_) = self.base.eat(Type::Dot) {
            return Ok(NodeBox::new_box(
                Box::new(NStringConcat {
                    left,
                    right: self.string_concat()?,
                }),
                NodeKind::StringConcat,
            ));
        }

        return Ok(left);
    }

    fn parenthese_statement(&mut self) -> Result<Box<NodeBox>, String> {
        if self.base.eat(Type::LParen).is_ok() {
            let content = self.condition_statement()?;
            self.base.eat(Type::RParen)?;

            return Ok(NodeBox::new_box(
                Box::new(NParentheseStatement { content }),
                NodeKind::ParentheseStatement,
            ));
        }

        return self.math_type();
    }

    fn basic_type(&mut self) -> Result<Box<NodeBox>, String> {
        if let Ok(math_type) = self.math_type() {
            return Ok(math_type);
        }
        if let Ok(array) = self.array() {
            return Ok(array);
        }
        if let Ok(string) = self.string() {
            return Ok(string);
        }

        return self.boolean();
    }

    fn math_type(&mut self) -> Result<Box<NodeBox>, String> {
        if let Ok(identifier) = self.identifer() {
            let call_statement = self.call_statement(Ok(identifier))?;
            return Ok(call_statement);
        }

        return self.number();
    }

    fn string_concat_type(&mut self) -> Result<Box<NodeBox>, String> {
        if let Ok(string) = self.string() {
            return Ok(string);
        }

        return self.math_statement();
    }

    fn identifer(&mut self) -> Result<Box<NodeBox>, String> {
        return Ok(NodeBox::new_box(
            Box::new(NIdentifier {
                name: self.base.eat(Type::Iden)?.val,
            }),
            NodeKind::Identifier,
        ));
    }

    fn array(&mut self) -> Result<Box<NodeBox>, String> {
        self.base.eat(Type::LAngleBracket)?;
        let mut items = vec![];
        if let Ok(condition) = self.condition() {
            items.push(condition);
            while let Ok(_) = self.base.eat(Type::Comma) {
                if let Ok(other_condition) = self.condition() {
                    items.push(other_condition);
                }
            }
        }

        self.base.eat(Type::RAngleBracket)?;
        return Ok(NodeBox::new_box(
            Box::new(NArray { items }),
            NodeKind::Array,
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
        let sign = self.base.eat_mult(&[Type::Add, Type::Min]);
        let num: f64 = self.base.eat(Type::Num)?.val.parse().unwrap();
        let sign_num;
        if sign.is_ok() {
            match sign.unwrap()._type {
                Type::Min => sign_num = -num,
                _ => sign_num = num,
            }
        } else {
            sign_num = num;
        }

        return Ok(NodeBox::new_box(
            Box::new(NNumber { val: sign_num }),
            NodeKind::Number,
        ));
    }

    fn boolean(&mut self) -> Result<Box<NodeBox>, String> {
        let boolean = self.base.eat_mult(&[Type::TrueK, Type::FalseK])?;
        let val = match boolean._type {
            Type::TrueK => true,
            Type::FalseK => false,
            _ => panic!("Not boolean"),
        };

        return Ok(NodeBox::new_box(
            Box::new(NBoolean { val }),
            NodeKind::Boolean,
        ));
    }
}
