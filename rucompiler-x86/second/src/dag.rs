use crate::ast::ASTNode;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum DAGNode {
    Number(usize),
    Identifier(String),
    Boolean(bool),
    Return((usize, Rc<DAGNode>)),
    Prog {
        argdecl: (usize, Rc<DAGNode>),
        typedecl: (usize, Rc<DAGNode>),
        stmts: (usize, Rc<DAGNode>),
        ret: (usize, Rc<DAGNode>),
    },
    ArgDecl(Vec<(usize, Rc<DAGNode>)>),
    TypeDecl {
        var_type: String,
        vars: Vec<(usize, Rc<DAGNode>)>,
    },
    Block(Vec<(usize, Rc<DAGNode>)>),
    Assign {
        var: (usize, Rc<DAGNode>),
        val: (usize, Rc<DAGNode>),
    },
    IfThenElse {
        cond: (usize, Rc<DAGNode>),
        true_block: (usize, Rc<DAGNode>),
        false_block: (usize, Rc<DAGNode>),
    },
    Whileloop {
        cond: (usize, Rc<DAGNode>),
        block: (usize, Rc<DAGNode>),
    },
    Binop {
        opr: String,
        lhs: (usize, Rc<DAGNode>),
        rhs: (usize, Rc<DAGNode>),
    },
}

fn get_valnum(node: DAGNode, value_nums: &mut HashMap<DAGNode, usize>, cur_valnum: &mut usize, dag_nodes: &mut HashMap<usize, Rc<DAGNode>>) -> (usize, Rc<DAGNode>) {
    if let Some(&vn) = value_nums.get(&node) {
        (vn, dag_nodes[&vn].clone())
    }
    else {
        let rc_node = Rc::new(node.clone());
        *cur_valnum += 1;
        value_nums.insert(node.clone(), *cur_valnum);
        dag_nodes.insert(*cur_valnum, rc_node.clone());
        (*cur_valnum, rc_node)
    }
}

pub fn dag_from_ast(root: &ASTNode, value_nums: &mut HashMap<DAGNode, usize>, cur_valnum: &mut usize, dag_nodes: &mut HashMap<usize, Rc<DAGNode>>) -> (usize, Rc<DAGNode>) {
    match root {
        ASTNode::Number(num) => {
            get_valnum(DAGNode::Number(*num), value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::Identifier(id) => {
            get_valnum(DAGNode::Identifier(id.to_string()), value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::Boolean(bool_val) => {
            get_valnum(DAGNode::Boolean(bool_val.clone()), value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::Return(var) => {
            let var_dag = dag_from_ast(&var, value_nums, cur_valnum, dag_nodes);
            get_valnum(DAGNode::Return(var_dag), value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::Prog{argdecl, typedecl, stmts, ret} => {
            // let mut argdecl_dags = Vec::new();
            // for arg in argdecl {
            //     argdecl_dags.push(dag_from_ast(&arg, value_nums, cur_valnum, dag_nodes));
            // }
            let argdecl_dag = dag_from_ast(&argdecl, value_nums, cur_valnum, dag_nodes);
            let typedecl_dag = dag_from_ast(&typedecl, value_nums, cur_valnum, dag_nodes);
            let stmts_dag = dag_from_ast(&stmts, value_nums, cur_valnum, dag_nodes);
            let ret_dag = dag_from_ast(&ret, value_nums, cur_valnum, dag_nodes);
            get_valnum(DAGNode::Prog{
                argdecl: argdecl_dag,
                typedecl: typedecl_dag,
                stmts: stmts_dag,
                ret: ret_dag,
            }, value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::ArgDecl(vars) => {
            let mut vars_dag = Vec::new();
            for var in vars {
                vars_dag.push(dag_from_ast(&var, value_nums, cur_valnum, dag_nodes));
            }
            get_valnum(DAGNode::ArgDecl(vars_dag), value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::TypeDecl{var_type, vars} => {
            let mut vars_dag = Vec::new();
            for var in vars {
                vars_dag.push(dag_from_ast(&var, value_nums, cur_valnum, dag_nodes));
            }
            get_valnum(DAGNode::TypeDecl{
                var_type: var_type.to_string(), 
                vars: vars_dag,
            }, value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::Block(vecs) => {
            // let stmt_dag = dag_from_ast(&stmt, value_nums, cur_valnum, dag_nodes);
            let mut vecs_dag = Vec::new();
            for vec in vecs {
                vecs_dag.push(dag_from_ast(&vec, value_nums, cur_valnum, dag_nodes));
            }
            get_valnum(DAGNode::Block(vecs_dag), value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::Assign{var, val} => {
            let var_dag = dag_from_ast(&var, value_nums, cur_valnum, dag_nodes);
            let val_dag = dag_from_ast(&val, value_nums, cur_valnum, dag_nodes);
            get_valnum(DAGNode::Assign{
                var: var_dag, 
                val: val_dag,
            }, value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::IfThenElse{cond, true_block, false_block} => {
            let cond_dag = dag_from_ast(&cond, value_nums, cur_valnum, dag_nodes);
            let true_block_dag = dag_from_ast(&true_block, value_nums, cur_valnum, dag_nodes);
            let false_block_dag = dag_from_ast(&false_block, value_nums, cur_valnum, dag_nodes);
            get_valnum(DAGNode::IfThenElse{
                cond: cond_dag, 
                true_block: true_block_dag, 
                false_block: false_block_dag,
            }, value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::Whileloop{cond, block} => {
            let cond_dag = dag_from_ast(&cond, value_nums, cur_valnum, dag_nodes);
            let block_dag = dag_from_ast(&block, value_nums, cur_valnum, dag_nodes);
            get_valnum(DAGNode::Whileloop {
                cond: cond_dag, 
                block: block_dag,
            }, value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::Binop{opr, lhs, rhs} => {
            let lhs_dag = dag_from_ast(&lhs, value_nums, cur_valnum, dag_nodes);
            let rhs_dag = dag_from_ast(&rhs, value_nums, cur_valnum, dag_nodes);
            get_valnum(DAGNode::Binop {
                opr: opr.to_string(), 
                lhs: lhs_dag, 
                rhs: rhs_dag,
            }, value_nums, cur_valnum, dag_nodes)
        }
    }
}

//TODO: Need implementation
// pub fn bfs_dag(root: &DAGNode, level: usize, traversed: &mut HashMap<usize, Vec<String>>) {
//     match root {
//         DAGNode::Number(num) => {
//             traversed.entry(level).or_default().push(num.to_string());
//         }
//         DAGNode::Identifier(id) => {
//             traversed.entry(level).or_default().push(id.to_string());
//         }
//         DAGNode::Binop{opr, lhs, rhs} => {
//             traversed.entry(level).or_default().push(opr.to_string());
//             let (_, left) = lhs;
//             let (_, right) = rhs;
//             bfs_dag(&*left.clone(), level + 1, traversed);
//             bfs_dag(&*right.clone(), level + 1, traversed);
//         } 
//     }
// }