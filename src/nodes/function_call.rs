use std::any::Any;

pub struct FunctionCall {
    pub func_name: String,
    pub parameters: Vec<Box<dyn Any>>,
}
