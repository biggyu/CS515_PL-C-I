use crate::dag::DAGNode;
use std::collections::HashMap;
use std::rc::Rc;

pub fn gen_llvm_ir(root: &DAGNode, temp_nums: &mut HashMap<DAGNode, usize>, dag_nodes: &HashMap<usize, Rc<DAGNode>>, func_type: Option<String>, func_name: Option<String>) -> String {
    let mut params: HashMap<Rc<DAGNode>, String> = HashMap::new();
    // function name
    let mut llvm_ir = format!("define {} @{}(", func_type.clone().unwrap_or_else(|| "i64".to_string()), func_name.unwrap_or_else(|| "foo".to_string()));
    // let mut chk: bool = false;
    // for node in dag_nodes.values() {
    //     match &**node {
    //         DAGNode::Identifier(id) => {
    //             chk = true;
    //             params.insert(node.clone(), format!("%{}", id));
    //             llvm_ir = llvm_ir + &func_type.clone().unwrap_or_else(|| "i64".to_string()) + &format!(" %{}, ", id).to_string();
    //         }
    //         _ => {} 
    //     }
    // }
    // if chk {
    //     llvm_ir.replace_range(llvm_ir.len() - 2..llvm_ir.len(), ") {\n");
    // }
    // else {
    //     llvm_ir = llvm_ir + &String::from(") {\n");
    // }
    // let mut ir = get_ir(root, temp_nums, &mut 0, func_type.clone(), &mut llvm_ir);
    // // return temporary
    // // llvm_ir = llvm_ir + &format!("\tret {} {}\n{}\n", &func_type.clone().unwrap_or_else(|| "i64".to_string()), ir, "}".to_string());
    // // llvm_ir.push_str(&ir);
    // llvm_ir
    get_ir(root, temp_nums, &mut 0, func_type.clone(), &mut llvm_ir);
    llvm_ir
}

fn is_reg(s: &str) -> bool {
    s.starts_with('%') && s[1..].chars().all(|c| c.is_alphabetic())
}

fn get_tempnum(node: DAGNode, temp_nums: &mut HashMap<DAGNode, usize>, cur_valnum: &mut usize) -> (usize, bool) {
    if let Some(&vn) = temp_nums.get(&node) {
        (vn, false)
    }
    else {
        // let rc_node = Rc::new(node.clone());
        *cur_valnum += 1;
        temp_nums.insert(node.clone(), *cur_valnum);
        (*cur_valnum, true)
    }
}

fn get_ir(root: &DAGNode, temp_nums: &mut HashMap<DAGNode, usize>, cur_valnum: &mut usize, func_type: Option<String>, llvm_ir: &mut String) -> String {
    match root {
        DAGNode::Number(num) => {
            num.to_string()
        }
        DAGNode::Identifier(id) => {
            format!("%{}", id)
        }
        DAGNode::Boolean(bool_val) => {
            bool_val.to_string()
        }
        DAGNode::Return(var) => {
            // let mut ret = format!("\tret {} ", func_type.clone().unwrap_or_else(|| "i64".to_string()));
            let (_, var_dag) = var;
            let var_ir = &get_ir(&*var_dag.clone(), temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            // let (_, var_chk) = get_tempnum((**var_dag).clone(), temp_nums, cur_valnum);
            llvm_ir.push_str(&format!("\t%t{} = load i64, ptr {}.alloc\n", *cur_valnum + 1, var_ir));
            llvm_ir.push_str(&format!("\tret {} %t{}\n", func_type.clone().unwrap_or_else(|| "i64".to_string()), *cur_valnum + 1));
            "\n".to_string()
            // format!("ret {} %{}", func_type.clone().unwrap_or_else(|| "i64".to_string()), var_tempnum)
        }
        DAGNode::Prog{argdecl, typedecl, stmts, ret} => {
            // let mut prog = String::new();
            let (_, argdecl_dag) = argdecl;
            let (_, typedecl_dag) = typedecl;
            let (_, stmts_dag) = stmts;
            let (_, ret_dag) = ret;
            let args_ir = &get_ir(&*argdecl_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            let _ = &get_ir(&*typedecl_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            llvm_ir.push_str(args_ir);
            let _ = &get_ir(&*stmts_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            let _ = &get_ir(&*ret_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            llvm_ir.push_str(&"}\n");
            "\n".to_string()
            // let (tempnum, chk) = get_tempnum(root.clone(), temp_nums, cur_valnum);
            // if chk {
            //     pro.push_str(&format!("\t%t{} = {} {} {}, {}\n", tempnum, opr_str, func_type.clone().unwrap_or_else(|| "i64".to_string()), left_temp, right_temp);
            // }
            // format!("%t{}", tempnum)

            // let argdecl_dag = get_ir(&*argdecl_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            // let typedecl_dag = get_ir(&*typedecl_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            // let stmts_dag = get_ir(&*stmts_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            // let ret_dag = get_ir(&*ret_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
        }
        DAGNode::ArgDecl(vars) => {
            let mut args = String::new();
            let mut args_alloc = String::new();
            for var in vars {
                let (_, var_dag) = var;
                let var_ir = get_ir(&*var_dag.clone(), temp_nums, cur_valnum, func_type.clone(), llvm_ir);
                llvm_ir.push_str(&format!("{} {}, ", func_type.clone().unwrap_or_else(|| "i64".to_string()), var_ir));
                args_alloc.push_str(&format!("\t{}.alloc = alloca {}\n", var_ir, func_type.clone().unwrap_or_else(|| "i64".to_string())));
                args.push_str(&format!("\tstore {} {}, ptr {}.alloc\n", func_type.clone().unwrap_or_else(|| "i64".to_string()), var_ir, var_ir));
            }
            if vars.len() > 0 {
                *llvm_ir = (&llvm_ir[..llvm_ir.len()-2]).to_string();
            }
            llvm_ir.push_str(&") {\nentry:\n");
            llvm_ir.push_str(&args_alloc);
            args
        }
        DAGNode::TypeDecl{var_type, vars} => {
            let mut types = String::new();
            for var in vars {
                let (_, var_dag) = var;
                let var_ir = get_ir(&*var_dag.clone(), temp_nums, cur_valnum, func_type.clone(), llvm_ir);
                llvm_ir.push_str(&format!("\t{}.alloc = alloca {}\n", var_ir, func_type.clone().unwrap_or_else(|| "i64".to_string())));
            }
            "\n".to_string()
        }
        DAGNode::Block(vecs) => {
            // let mut blocks = String::new();
            for vec in vecs {
                let (_, vec_dag) = vec;
                let _ = &get_ir(&*vec_dag.clone(), temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            }
            "\n".to_string()
        }
        DAGNode::Assign{var, val} => {
            // let mut assign = String::new();
            let (_, var_dag) = var;
            let (_, val_dag) = val;
            let var_ir = &get_ir(&*var_dag.clone(), temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            let val_ir = &get_ir(&*val_dag.clone(), temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            llvm_ir.push_str(&format!("\tstore {} {}, ptr {}.alloc\n", func_type.clone().unwrap_or_else(|| "i64".to_string()), val_ir, var_ir));
            "\n".to_string()
        }
        DAGNode::IfThenElse{cond, true_block, false_block} => {
            // let mut ifthenelse = String::new();
            let (_, cond_dag) = cond;
            let (_, true_block_dag) = true_block;
            let (_, false_block_dag) = false_block;
            let cond_ir = get_ir(&*cond_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            let label_num = *cur_valnum;
            *cur_valnum += 1;
            llvm_ir.push_str(&format!("\tbr i1 {}, label %if{}.then, label %if{}.else\n\nif{}.then:\n", cond_ir, label_num, label_num, label_num));
            let _ = get_ir(&*true_block_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            llvm_ir.push_str(&format!("\tbr label %if{}.end\n\nif{}.else:\n", label_num, label_num));
            let _ = get_ir(&*false_block_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            llvm_ir.push_str(&format!("\tbr label %if{}.end\n\nif{}.end:\n", label_num, label_num));
            // llvm_ir.push_str(&format!("{}\n", cond_ir));
            // llvm_ir.push_str(&format!("{}\n", true_block_ir));
            "\n".to_string()
        }
        DAGNode::Whileloop{cond, block} => {
            // let mut whileloop = String::new();
            let (_, cond_dag) = cond;
            let (_, block_dag) = block;
            // let cond_ir = get_ir(&*cond_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            // let block_ir = get_ir(&*block_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            let label_num = *cur_valnum;
            *cur_valnum += 1;
            llvm_ir.push_str(&format!("\tbr label %while{}.cond\n\nwhile{}.cond:\n", label_num, label_num));
            let cond_ir = get_ir(&*cond_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            // llvm_ir.push_str(&format!("{}\n", block_ir));
            llvm_ir.push_str(&format!("\tbr i1 {}, label %while{}.body, label %while{}.end\n\nwhile{}.body:\n", cond_ir, label_num, label_num, label_num));
            let _ = get_ir(&*block_dag, temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            llvm_ir.push_str(&format!("\tbr label %while{}.cond\n\nwhile{}.end:\n", label_num, label_num));
            "\n".to_string()
        }
        DAGNode::Binop{opr, lhs, rhs} => {
            // let mut binop = String::new();
            let (_, lhs_dag) = lhs;
            let (_, rhs_dag) = rhs;
            let mut lhs_ir = get_ir(&*lhs_dag.clone(), temp_nums, cur_valnum, func_type.clone(), llvm_ir);
            let mut rhs_ir = get_ir(&*rhs_dag.clone(), temp_nums, cur_valnum, func_type.clone(), llvm_ir);

            // let (lhs_num, lhs_chk) = get_tempnum((**lhs_dag).clone(), temp_nums, cur_valnum);
            // let (rhs_num, rhs_chk) = get_tempnum((**rhs_dag).clone(), temp_nums, cur_valnum);
            // let (root_num, root_chk) = get_tempnum(root.clone(), temp_nums, cur_valnum);
            let mut result_tmp = String::new();
            let mut opr_str = "";
            *cur_valnum += 1;
            let root_num = *cur_valnum;
            if opr == "+" {
                result_tmp = format!("%t{}", root_num);
                opr_str = "add";
            }
            else if opr == "*" {
                result_tmp = format!("%t{}", root_num);
                opr_str = "mul";
            }
            else if opr == "<" {
                result_tmp = format!("%cmp{}", root_num);
                opr_str = "icmp ult"; 
            }
            else if opr == "<=" {
                result_tmp = format!("%cmp{}", root_num);
                opr_str = "icmp ule"; 
            }
            else if opr == ">" {
                result_tmp = format!("%cmp{}", root_num);
                opr_str = "icmp ugt"; 
            }
            else if opr == ">=" {
                result_tmp = format!("%cmp{}", root_num);
                opr_str = "icmp uge"; 
            }
            else if opr == "==" {
                result_tmp = format!("%cmp{}", root_num);
                opr_str = "icmp eq"; 
            }
            if is_reg(&lhs_ir) {
                *cur_valnum += 1;
                let lhs_num = *cur_valnum;
                llvm_ir.push_str(&format!("\t%t{} = load {}, ptr {}.alloc\n", lhs_num, func_type.clone().unwrap_or_else(|| "i64".to_string()), lhs_ir));
                lhs_ir = format!("%t{}", lhs_num);
            }
            if is_reg(&rhs_ir) {
                *cur_valnum += 1;
                let rhs_num = *cur_valnum;
                llvm_ir.push_str(&format!("\t%t{} = load {}, ptr {}.alloc\n", rhs_num, func_type.clone().unwrap_or_else(|| "i64".to_string()), rhs_ir));
                rhs_ir = format!("%t{}", rhs_num);
            }
            llvm_ir.push_str(&format!("\t{} = {} {} {}, {}\n", result_tmp, opr_str, func_type.clone().unwrap_or_else(|| "i64".to_string()), lhs_ir, rhs_ir));
            result_tmp.to_string()
        }
    }
}
