use crate::parser::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(usize),
    Identifier(String),
    Binop {
        opr: String,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
}

pub fn gen_ast(parse_node: &ParseNode) -> Result<ASTNode, String> {
    match parse_node.token.as_str() {
        "EXPR" => {
            let term_node = gen_ast(&parse_node.children[0].clone().unwrap())?;
            gen_exprdash(&parse_node.children[1].clone().unwrap(), term_node)
        }
        "EXPRDASH" => Err("EXPRDASH shouldn't be handled individually".to_string()),
        "TERM" => {
            let factor_node = gen_ast(&parse_node.children[0].clone().unwrap())?;
            gen_termdash(&parse_node.children[1].clone().unwrap(), factor_node)
        }
        "TERMDASH" => Err("TERMDASH shouldn't be handled individually".to_string()),
        "FACTOR" => {
            match parse_node.children[0].clone().unwrap().token.as_str() {
                "IDENTIFIER" => {
                    // let id_name = parse_node.children[0].clone().value.clone().unwrap();
                    Ok(ASTNode::Identifier(parse_node.children[0].clone().unwrap().value.clone().unwrap()))
                }
                "NUMBER" => {
                    // let val = parse_node.children[0].clone().value.clone().unwrap();
                    let num = parse_node.children[0].clone().unwrap().value.clone().unwrap().parse::<usize>().map_err(|e| e.to_string())?;
                    Ok(ASTNode::Number(num))
                }
                "(" => {
                    gen_ast(&parse_node.children[1].clone().unwrap())
                }
                _ => Err(format!(
                    "Unidentified FACTOR rule {:?}", parse_node.children[1].clone().unwrap()
                )),
            }
        }
        _ => Err(format!(
            "Unidentified symbol {}", parse_node.token
        )),
    }
}

pub fn gen_exprdash(node: &ParseNode, inh_attr: ASTNode) -> Result<ASTNode, String> {
    if node.children.is_empty() || node.children[0].clone().unwrap().token == "EPSILON" {
        Ok(inh_attr)
    }
    else {
        let term_node = gen_ast(&node.children[1].clone().unwrap())?;
        let new_node = ASTNode::Binop {
            opr: "+".to_string(),
            lhs: Box::new(inh_attr),
            rhs: Box::new(term_node),
        };
        gen_exprdash(&node.children[2].clone().unwrap(), new_node)
    }
}

pub fn gen_termdash(node: &ParseNode, inh_attr: ASTNode) -> Result<ASTNode, String> {
    if node.children.is_empty() || node.children[0].clone().unwrap().token == "EPSILON" {
        Ok(inh_attr)
    }
    else {
        let factor_node = gen_ast(&node.children[1].clone().unwrap())?;
        let new_node = ASTNode::Binop {
            opr: "*".to_string(),
            lhs: Box::new(inh_attr),
            rhs: Box::new(factor_node),
        };
        gen_termdash(&node.children[2].clone().unwrap(), new_node)
    }
}

pub fn bfs_ats(root: &ASTNode, level: usize, traversed: &mut HashMap<usize, Vec<String>>) {
    match root {
        ASTNode::Number(num) => {
            traversed.entry(level).or_default().push(num.to_string());
        }
        ASTNode::Identifier(id) => {
            traversed.entry(level).or_default().push(id.to_string());
        }
        ASTNode::Binop{opr, lhs, rhs} => {
            traversed.entry(level).or_default().push(opr.to_string());
            bfs_ats(&lhs, level + 1, traversed);
            bfs_ats(&rhs, level + 1, traversed);
        }
        // _ => println!("No ASTNode"),
    }
}