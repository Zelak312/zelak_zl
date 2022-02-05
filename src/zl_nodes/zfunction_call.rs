use std::any::Any;

pub struct ZFunctionCall {
    pub func_name: String,
    pub parameters: Vec<Box<dyn Any>>,
}
