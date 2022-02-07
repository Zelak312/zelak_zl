use std::any::Any;
pub struct NProgram {
    pub childs: Vec<Box<dyn Any>>,
}
