use std::any::Any;

pub struct BEcho {
    pub to_echo: Box<dyn Any>,
}
