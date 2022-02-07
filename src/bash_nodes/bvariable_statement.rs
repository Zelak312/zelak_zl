use std::any::Any;
pub struct BVariableStatement {
    pub iden: Box<dyn Any>,
    pub expression: Box<dyn Any>,
}
