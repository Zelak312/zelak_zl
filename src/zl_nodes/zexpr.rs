use std::any::Any;
pub struct ZExpr {
    pub childs: Vec<Box<dyn Any>>,
}
