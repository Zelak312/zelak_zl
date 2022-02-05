use std::any::Any;

pub struct ZFunction_call {
    pub func_name: String,
    pub parameters: Vec<Box<dyn Any>>,
}
