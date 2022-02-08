use super::{
    node_kind::NodeKind,
    nodes::{
        call_statement::NCallStatement, condition::NCondition,
        condition_statement::NConditionStatement, expression_statement::NExpressionStatement,
        identifier::NIdentifier, if_statement::NIfStatement, math_statement::NMathStatement,
        number::NNumber, program::NProgram, string::NString,
        variable_statement::NVariableStatement,
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
            NodeKind::IfStatement => {
                let d = self.content.downcast_ref::<NIfStatement>().unwrap();
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
            NodeKind::MathStatement => {
                let d = self.content.downcast_ref::<NMathStatement>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::Identifier => {
                let d = self.content.downcast_ref::<NIdentifier>().unwrap();
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
        }
    }
}
