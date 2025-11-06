use crate::ast::ASTNode;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct CFGNode {
    id: usize,
    inst: Vec<ASTNode>,
    succ: Vec<usize>,
}

pub fn gen_cfg(root: &ASTNode) -> HashMap<usize, CFGNode> {
    let mut cfg: HashMap<usize, CFGNode> = HashMap::new();
    let mut id_counter = 0;
    let entry_node = cfg_traverse(root, &mut cfg, &mut id_counter);

    cfg
}

fn cfg_traverse(root: &ASTNode, cfg: &mut HashMap<usize, CFGNode>, id_counter: &mut usize) -> (usize, usize) {
    match root {
        ASTNode::Prog{argdecl, typedecl, stmts, ret} => {
            let (stmts_id, stmts_id_last) = cfg_traverse(&stmts, cfg, id_counter);
            let (ret_id, ret_id_last) = cfg_traverse(&ret, cfg, id_counter);
            // cfg.nodes.get_mut(&stmts_id).unwrap().succ.push(ret_id);
            cfg.get_mut(&stmts_id_last).unwrap().succ.push(ret_id);
            (stmts_id, ret_id_last)
        }
        ASTNode::Block(stmts) => {
            let mut first_id = None;
            let mut last_id = None;
            for (idx, stmt) in stmts.into_iter().enumerate() {
                let (node_id, node_id_end) = cfg_traverse(&stmt, cfg, id_counter);
                if let Some(prev_id) = last_id {
                    // cfg.nodes.get_mut(&prev_id).unwrap().succ.push(node_id);
                    // println!("{} {}", prev_id, node_id);
                    cfg.get_mut(&prev_id).unwrap().succ.push(node_id);
                }
                if idx == 0 {
                    first_id = Some(node_id);
                }
                last_id = Some(node_id_end);
            }
            (first_id.unwrap(), last_id.unwrap())
        }
        ASTNode::Assign{var, val} => {
            let id = *id_counter;
            *id_counter += 1;
            cfg.insert(id, CFGNode {
                id: id,
                inst: vec![root.clone()],
                succ: Vec::new(),
            });
            (id, id)
        }
        ASTNode::IfThenElse{cond, true_block, false_block} => {
            let start_id = *id_counter;
            *id_counter += 1;
            let mut cond_node = CFGNode {
                id: start_id,
                inst: vec![*cond.clone()],
                succ: Vec::new(),
            };
            let (true_block_id, true_block_id_last) = cfg_traverse(true_block, cfg, id_counter);
            let (false_block_id, false_block_id_last) = cfg_traverse(false_block, cfg, id_counter);
            cond_node.succ.push(true_block_id);
            cond_node.succ.push(false_block_id);
            cfg.insert(start_id, cond_node);
            let end_id = *id_counter;
            *id_counter += 1;
            let mut end_node = CFGNode {
                id: end_id,
                inst: Vec::new(),
                succ: Vec::new(),
            };
            // cfg.get_mut(&true_block_id_last).unwrap().succ.push(end_id);
            cfg.get_mut(&true_block_id_last).unwrap().succ.push(end_id);
            // cfg.get_mut(&false_block_id_last).unwrap().succ.push(end_id);
            cfg.get_mut(&false_block_id_last).unwrap().succ.push(end_id);
            cfg.insert(end_id, end_node);
            (start_id, end_id)
        }
        ASTNode::Whileloop{cond, block} => {
            let loop_id = *id_counter;
            *id_counter += 1;
            let mut cond_node = CFGNode {
                id: loop_id,
                inst: vec![*cond.clone()],
                succ: Vec::new(),
            };
            let (block_id, block_id_end) = cfg_traverse(block, cfg, id_counter);
            cond_node.succ.push(block_id);
            // cfg.get_mut(&loop_id).unwrap().succ.push(block_id);
            cfg.get_mut(&block_id_end).unwrap().succ.push(loop_id);
            cfg.insert(loop_id, cond_node);
            (loop_id, block_id_end)
        }
        ASTNode::Return(var) => {
            let id = *id_counter;
            *id_counter += 1;
            cfg.insert(id, CFGNode {
                id: id,
                inst: vec![root.clone()],
                succ: Vec::new(),
            });
            (id, id)
        }
        _ => {
            (0, 0)
        }
    }
}

pub fn compute_dom (cfg: &HashMap<usize, CFGNode>, entry: usize) -> HashMap<usize, HashSet<usize>> {
    let mut dom: HashMap<usize, HashSet<usize>> = HashMap::new();
    for node_id in cfg.keys() {
        if *node_id == entry {
            dom.insert(*node_id, vec![*node_id].into_iter().collect());
        }
        else {
            dom.insert(*node_id, cfg.keys().cloned().collect());
        }   
    }

    let mut changed = true;
    let mut ordered_keys: Vec<_> = cfg.keys().into_iter().collect();
    ordered_keys.sort_by(|x, y| x.cmp(&y));
    while changed {
        changed = false;
        for node_id in &ordered_keys {
            if **node_id == entry {
                continue;
            }
            let preds: Vec<usize> = cfg.iter().filter(|(_, node)| node.succ.contains(&node_id)).map(|(id, _)| *id).collect();
            let mut new_dom = preds.iter().filter_map(|p| dom.get(p)).cloned().reduce(|a, b| &a & &b).unwrap_or_default();
            new_dom.insert(**node_id);
    
            if new_dom != dom[&node_id] {
                dom.insert(**node_id, new_dom);
                changed = true;
            }
        }
    }
    dom
}

pub fn compute_idom(dom: &HashMap<usize, HashSet<usize>>, entry: usize) -> HashMap<usize, usize> {
    let mut idom = HashMap::new();
    for (&node_id, doms) in dom.iter() {
        if node_id == entry {
            continue;
        }
        let mut sdoms = doms.clone();
        sdoms.remove(&node_id);
        for &d in sdoms.clone().iter() {
            let mut other_doms = sdoms.clone();
            other_doms.remove(&d);
            if other_doms.is_subset(&dom[&d]) {
                idom.insert(node_id, d);
                break;
            }
        }
    }
    idom
}

pub fn compute_df(cfg: &HashMap<usize, CFGNode>, idom: &HashMap<usize, usize>) -> HashMap<usize, HashSet<usize>> {
    let mut df: HashMap<usize, HashSet<usize>> = HashMap::new();
    for &node_id in cfg.keys() {
        df.insert(node_id, HashSet::new());
    }

    let mut dom_children: HashMap<usize, Vec<usize>> = HashMap::new();
    for &node_id in cfg.keys() {
        dom_children.insert(node_id, Vec::new());
    }
    for (&node, &parent) in idom.iter() {
        if node != parent {
            if let Some(children) = dom_children.get_mut(&parent) {
                children.push(node);
            }
        }
    }

    let mut ordered_keys: Vec <_> = idom.keys().into_iter().collect();
    ordered_keys.sort_by(|x, y| y.cmp(&x));
    // X = key
    for key in ordered_keys {
        // Y = node_id
        for node_id in &cfg[key].succ.clone() {
            if idom[node_id] != *key {
                if let Some(val) = df.get_mut(key) {
                    val.insert(*node_id);
                }
            }
        }
        for z in &dom_children[key] {
            for y in df[&z].clone() {
                if idom[&y] != *key {
                    if let Some(val) = df.get_mut(key) {
                        val.insert(y);
                    }
                }
            }
        }
    }
    df
}