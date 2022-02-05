use std::any::Any;

pub struct ZBinOp {
    pub op: String,
    pub parenthese: bool,
    pub left: Box<dyn Any>,
    pub right: Box<dyn Any>,
}
