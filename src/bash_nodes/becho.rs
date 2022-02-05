use std::any::Any;

pub struct BEcho {
    pub to_echo: Vec<Box<dyn Any>>,
}
