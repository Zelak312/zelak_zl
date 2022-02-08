use super::{
    node_kind::NodeKind,
    nodes::{
        expression_statement::NExpressionStatement, identifier::NIdentifier, number::NNumber,
        program::NProgram, variable_statement::NVariableStatement,
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

    pub fn debug(self, tab: Option<usize>) {
        let tab_r = tab.unwrap_or(0);
        match self._type {
            NodeKind::Program => {
                let d = self.content.downcast::<NProgram>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::VariableStatement => {
                let d = self.content.downcast::<NVariableStatement>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::ExpressionStatement => {
                let d = self.content.downcast::<NExpressionStatement>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::Identifier => {
                let d = self.content.downcast::<NIdentifier>().unwrap();
                d.debug(tab_r)
            }
            NodeKind::Number => {
                let d = self.content.downcast::<NNumber>().unwrap();
                d.debug(tab_r)
            }
        }
    }
}
