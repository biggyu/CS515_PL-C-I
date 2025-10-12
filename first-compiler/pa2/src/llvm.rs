use crate::dag::DAGNode;
use std::rc::Rc;
pub fn gen_llvm_ir(root: &Rc<DAGNode>) -> String {
    let mut llvm_ir = String::new();

    llvm_ir
}