use std::any::Any;
pub struct BVariableStatement {
    pub identifier: Box<dyn Any>,
    pub expression: Box<dyn Any>,
}
