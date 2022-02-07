use crate::token::Type;
use std::any::Any;

use super::identifier::NIdentifier;

pub struct NVariableStatement {
    pub declare_type: Option<Type>,
    pub identifier: Box<NIdentifier>,
    pub expression: Box<dyn Any>,
}
