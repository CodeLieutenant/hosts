use crate::cst::CstNode;

pub trait CstVisitor {
    fn visit(&mut self, node: &CstNode);
}
