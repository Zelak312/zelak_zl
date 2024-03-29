use crate::bash_nodes::{
    bmath_expression::BMathExpression, bstring_concat_expression::BStringConcatExpression,
};

use super::{
    node_kind::NodeKind,
    nodes::{
        array::NArray, boolean::NBoolean, call_statement::NCallStatement, condition::NCondition,
        condition_statement::NConditionStatement, expression_statement::NExpressionStatement,
        for_statement::NForStatement, function_definition::NFunctionDefinition,
        function_return::NFunctionReturn, identifier::NIdentifier, if_statement::NIfStatement,
        math_statement::NMathStatement, number::NNumber,
        parenthese_statement::NParentheseStatement, program::NProgram, string::NString,
        string_concat::NStringConcat, variable_statement::NVariableStatement,
    },
};
use std::any::Any;
pub struct NodeBox {
    pub content: Box<dyn Any>,
    pub _type: NodeKind,
}

impl NodeBox {
    pub fn new(content: Box<dyn Any>, _type: NodeKind) -> Self {
        Self { content, _type }
    }

    pub fn new_box(content: Box<dyn Any>, _type: NodeKind) -> Box<Self> {
        Box::new(NodeBox::new(content, _type))
    }

    pub fn debug(&self, tab: Option<usize>) {
        let tab_r = tab.unwrap_or(0);
        match self._type {
            NodeKind::Program => {
                let d = self.content.downcast_ref::<NProgram>().unwrap();
                d.debug(tab_r);
            }
            NodeKind::FunctionDefinition => {
                let d = self.content.downcast_ref::<NFunctionDefinition>().unwrap();
                d.debug(tab_r);
            }
            NodeKind::FunctionReturn => {
                let d = self.content.downcast_ref::<NFunctionReturn>().unwrap();
                d.debug(tab_r);
            }
            NodeKind::IfStatement => {
                let d = self.content.downcast_ref::<NIfStatement>().unwrap();
                d.debug(tab_r);
            }
            NodeKind::ForStatement => {
                let d = self.content.downcast_ref::<NForStatement>().unwrap();
                d.debug(tab_r);
            }
            NodeKind::ConditionStatement => {
                let d = self.content.downcast_ref::<NConditionStatement>().unwrap();
                d.debug(tab_r);
            }
            NodeKind::Condition => {
                let d = self.content.downcast_ref::<NCondition>().unwrap();
                d.debug(tab_r);
            }
            NodeKind::VariableStatement => {
                let d = self.content.downcast_ref::<NVariableStatement>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::ExpressionStatement => {
                let d = self.content.downcast_ref::<NExpressionStatement>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::CallStatement => {
                let d = self.content.downcast_ref::<NCallStatement>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::ParentheseStatement => {
                let d = self.content.downcast_ref::<NParentheseStatement>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::MathStatement => {
                let d = self.content.downcast_ref::<NMathStatement>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::StringConcat => {
                let d = self.content.downcast_ref::<NStringConcat>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::Identifier => {
                let d = self.content.downcast_ref::<NIdentifier>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::Array => {
                let d = self.content.downcast_ref::<NArray>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::String => {
                let d = self.content.downcast_ref::<NString>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::Number => {
                let d = self.content.downcast_ref::<NNumber>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::Boolean => {
                let d = self.content.downcast_ref::<NBoolean>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::BMathExpression => {
                let d = self.content.downcast_ref::<BMathExpression>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::BStringConcatExpression => {
                let d = self
                    .content
                    .downcast_ref::<BStringConcatExpression>()
                    .unwrap();
                d.debug(tab_r)
            }
        }
    }
}
