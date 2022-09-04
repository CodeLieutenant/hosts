use crate::cst::CstNode;


pub trait CstVisitor {
    fn visit(&mut self, i: usize, node: &CstNode) -> Option<()>;
}
