use std::any::Any;

pub struct BAssignment {
    pub iden: String,
    pub content: Box<dyn Any>,
}
