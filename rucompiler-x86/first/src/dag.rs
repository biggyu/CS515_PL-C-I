use crate::ast::ASTNode;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum DAGNode {
    Number(usize),
    Identifier(String),
    Binop {
        opr: String,
        lhs: (usize, Rc<DAGNode>),
        rhs: (usize, Rc<DAGNode>),
    },
}

pub fn get_valnum(node: DAGNode, value_nums: &mut HashMap<DAGNode, usize>, cur_valnum: &mut usize, dag_nodes: &mut HashMap<usize, Rc<DAGNode>>) -> (usize, Rc<DAGNode>) {
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

pub fn dag_rep(root: &ASTNode, value_nums: &mut HashMap<DAGNode, usize>, cur_valnum: &mut usize, dag_nodes: &mut HashMap<usize, Rc<DAGNode>>) -> (usize, Rc<DAGNode>) {
    match root {
        ASTNode::Number(num) => {
            get_valnum(DAGNode::Number(*num), value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::Identifier(id) => {
            get_valnum(DAGNode::Identifier(id.to_string()), value_nums, cur_valnum, dag_nodes)
        }
        ASTNode::Binop{opr, lhs, rhs} => {
            let left = dag_rep(&lhs, value_nums, cur_valnum, dag_nodes);
            let right = dag_rep(&rhs, value_nums, cur_valnum, dag_nodes);
            get_valnum(DAGNode::Binop{opr: opr.to_string(), lhs: left, rhs: right,}, value_nums, cur_valnum, dag_nodes)
        }
    }
}

pub fn bfs_dag(root: &DAGNode, level: usize, traversed: &mut HashMap<usize, Vec<String>>, vars: &mut String) {
    match root {
        DAGNode::Number(num) => {
            traversed.entry(level).or_default().push(num.to_string());
        }
        DAGNode::Identifier(id) => {
            vars.push_str(&format!("{},", id));
            traversed.entry(level).or_default().push(id.to_string());
        }
        DAGNode::Binop{opr, lhs, rhs} => {
            traversed.entry(level).or_default().push(opr.to_string());
            let (_, left) = lhs;
            let (_, right) = rhs;
            bfs_dag(&*left.clone(), level + 1, traversed, vars);
            bfs_dag(&*right.clone(), level + 1, traversed, vars);
        } 
    }
}