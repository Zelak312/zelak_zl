use crate::token::Type;
use std::any::Any;

pub struct Assignment {
    pub declare_type: Option<Type>,
    pub iden: String,
    pub content: Box<dyn Any>,
}
