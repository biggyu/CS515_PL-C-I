use crate::dag::DAGNode;
use std::collections::HashMap;
use std::rc::Rc;
// #[derive(Debug, Clone)]
// pub struct Temporary {
//     pub value_num: usize,

// }
pub fn gen_llvm_ir(root: &DAGNode, temp_nums: &mut HashMap<DAGNode, usize>, dag_nodes: &HashMap<usize, Rc<DAGNode>>, args: &String, func_type: Option<String>, func_name: Option<String>) -> String {
    // function name
    let mut llvm_ir = format!("define {} @{}(", func_type.clone().unwrap_or_else(|| "i64".to_string()), func_name.unwrap_or_else(|| "foo".to_string()));
    let mut chk: bool = false;
    for var in args.split(",") {
        // params.insert(node.clone(), format!("%{}", var));
        if var != "" {
            chk = true;
            llvm_ir.push_str(&format!("{} %{}, ", func_type.clone().unwrap_or_else(|| "i64".to_string()), var))
        }
    }
    if chk {
        llvm_ir.replace_range(llvm_ir.len() - 2..llvm_ir.len(), ") {\n");
    }
    else {
        llvm_ir = llvm_ir + &String::from(") {\n");
    }
    let mut ir = get_ir(root, temp_nums, &mut 0, func_type.clone(), &mut llvm_ir);
    // return temporary
    llvm_ir = llvm_ir + &format!("\tret {} {}\n{}\n", &func_type.clone().unwrap_or_else(|| "i64".to_string()), ir, "}".to_string());
    llvm_ir
}

pub fn get_tempnum(node: DAGNode, temp_nums: &mut HashMap<DAGNode, usize>, cur_valnum: &mut usize) -> (usize, bool) {
    if let Some(&vn) = temp_nums.get(&node) {
        (vn, false)
    }
    else {
        let rc_node = Rc::new(node.clone());
        *cur_valnum += 1;
        temp_nums.insert(node.clone(), *cur_valnum);
        (*cur_valnum, true)
    }
}

pub fn get_ir(root: &DAGNode, temp_nums: &mut HashMap<DAGNode, usize>, cur_valnum: &mut usize, func_type: Option<String>, llvm_ir: &mut String) -> String {
    match root {
        DAGNode::Number(num) => {
            num.to_string()
        }
        DAGNode::Identifier(id) => {
            format!("%{}", id)
        }
        DAGNode::Binop{opr, lhs, rhs} => {
            let (_, left) = lhs;
            let (_, right) = rhs;
            let left_temp = get_ir(&*left.clone(), temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            let right_temp = get_ir(&*right.clone(), temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            let (tempnum, chk) = get_tempnum(root.clone(), temp_nums, cur_valnum);
            let mut opr_str = "";
            if opr == "+" {
                opr_str = "add";
            }
            else {
                opr_str = "mul";
            }
            if chk {
                *llvm_ir = llvm_ir.to_owned() + &format!("\t%t{} = {} {} {}, {}\n", tempnum, opr_str, func_type.clone().unwrap_or_else(|| "i64".to_string()), left_temp, right_temp);
            }
            format!("%t{}", tempnum)
            // format!("%t{}", get_tempnum(root.clone(), temp_nums, cur_valnum))
        }
    }
}
