use crate::parser::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(usize),
    Identifier(String),
    Boolean(bool),
    Semicolon(String),
    Prog {
        argdecl: Box<ASTNode>,
        typedecl: Box<ASTNode>,
        stmts: Box<ASTNode>,
        ret: Box<ASTNode>,
    },
    Return {
        opr: String,
        var: Box<ASTNode>,
    },
    ArgDecl {
        opr: String,
        var: Box<ASTNode>,
        tail: Box<ASTNode>,
    },
    TypeDecl {
        var_type: String,
        var: Box<ASTNode>,
        tail: Box<ASTNode>,
    },
    Stmts {
        stmt: Box<ASTNode>,
        stmts: Box<ASTNode>,
    },
    Assign {
        var: Box<ASTNode>,
        val: Box<ASTNode>,
    },
    IfThenElse {
        cond: Box<ASTNode>,
        true_block: Box<ASTNode>,
        false_block: Box<ASTNode>,
    },
    Whileloop {
        cond: Box<ASTNode>,
        block: Box<ASTNode>,
    },
    Relop {
        opr: String,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    Binop {
        opr: String,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
}

pub fn gen_ast(parse_node: &ParseNode) -> Result<ASTNode, String> {
    match parse_node.token.as_str() {
        "PROG" => {
            let argdecl_node = gen_ast(&parse_node.children[0].clone().unwrap())?;
            let typedecl_node = gen_ast(&parse_node.children[1].clone().unwrap())?;
            let stmts_node = gen_ast(&parse_node.children[2].clone().unwrap())?;
            let ret_node = gen_ast(&parse_node.children[3].clone().unwrap())?;
            Ok(ASTNode::Prog {
                argdecl: Box::new(argdecl_node),
                typedecl: Box::new(typedecl_node),
                stmts: Box::new(stmts_node),
                ret: Box::new(ret_node),
            })
        }
        "RET" => {
            Ok(ASTNode::Return {
                opr: parse_node.children[0].clone().unwrap().token,
                var: Box::new(ASTNode::Identifier(parse_node.children[1].clone().unwrap().value.clone().unwrap())),
            })
        }
        "ARGDECL" => {
            let argdecltail_node = gen_ast(&parse_node.children[2].clone().unwrap())?;
            Ok(ASTNode::ArgDecl {
                opr: parse_node.children[0].clone().unwrap().token,
                var: Box::new(ASTNode::Identifier(parse_node.children[1].clone().unwrap().value.clone().unwrap())),
                tail: Box::new(argdecltail_node),
            })
        }
        "ARGDECLTAIL" => {
            match parse_node.children[0].clone().unwrap().token.as_str() {
                ";" => {
                    Ok(ASTNode::Semicolon(";".to_string()))
                }
                _ => {
                    let argdecltail_node = gen_ast(&parse_node.children[2].clone().unwrap())?;
                    Ok(ASTNode::ArgDecl {
                        opr: "".to_string(),
                        var: Box::new(ASTNode::Identifier(parse_node.children[0].clone().unwrap().value.clone().unwrap())),
                        tail: Box::new(argdecltail_node),
                    })
                }
            }
        }
        "TYPEDECL" => {
            let typedecltail_node = gen_ast(&parse_node.children[2].clone().unwrap())?;
            Ok(ASTNode::TypeDecl {
                var_type: parse_node.children[0].clone().unwrap().token,
                var: Box::new(ASTNode::Identifier(parse_node.children[1].clone().unwrap().value.clone().unwrap())),
                tail: Box::new(typedecltail_node),
            })
            // gen_typedecltail(&parse_node.children[0].clone().unwrap(), &parse_node.children[1].clone().unwrap(), &parse_node.children[2].clone().unwrap())
        }
        "TYPEDECLTAIL" => {
            match parse_node.children[0].clone().unwrap().token.as_str() {
                ";" => {
                    Ok(ASTNode::Semicolon(";".to_string()))
                }
                "," => {
                    let typedecltail_node = gen_ast(&parse_node.children[2].clone().unwrap())?;
                    Ok(ASTNode::TypeDecl {
                        var_type: ",".to_string(),
                        var: Box::new(ASTNode::Identifier(parse_node.children[1].clone().unwrap().value.clone().unwrap())),
                        tail: Box::new(typedecltail_node),
                    })
                }
                _ => Err(format!(
                    "Unidentified TYPEDECLTAIL rule {:?}", parse_node.children[1].clone().unwrap()
                )),
            }
        }
        "STMTS" => {
            let stmt_node = gen_ast(&parse_node.children[0].clone().unwrap())?;
            gen_stmts(&parse_node.children[1].clone().unwrap(), stmt_node)
            // let stmts_node = gen_ast(&parse_node.children[1].clone().unwrap())?;
            // Ok(ASTNode::Stmts {
            //     stmt: Box::new(stmt_node),
            //     stmts: Box::new(stmts_node),
            // })
        }
        "STMT" => {
            gen_ast(&parse_node.children[0].clone().unwrap())
        }
        "ASSIGN" => {
            let expr_node = gen_ast(&parse_node.children[2].clone().unwrap())?;
            Ok(ASTNode::Assign {
                var: Box::new(ASTNode::Identifier(parse_node.children[0].clone().unwrap().value.clone().unwrap())),
                val: Box::new(expr_node),
            })
        }
        "IFTHENELSE" => {
            let bool_node = gen_ast(&parse_node.children[1].clone().unwrap())?;
            let stmts1_node = gen_ast(&parse_node.children[4].clone().unwrap())?;
            let stmts2_node = gen_ast(&parse_node.children[8].clone().unwrap())?;
            Ok(ASTNode::IfThenElse {
                cond: Box::new(bool_node),
                true_block: Box::new(stmts1_node),
                false_block: Box::new(stmts2_node),
            })
        }
        "WHILE" => {
            let bool_node = gen_ast(&parse_node.children[1].clone().unwrap())?;
            let stmts_node = gen_ast(&parse_node.children[5].clone().unwrap())?;
            Ok(ASTNode::Whileloop{
                cond: Box::new(bool_node),
                block: Box::new(stmts_node),
            })
        }
        "BOOL" => {
            match parse_node.children[0].clone().unwrap().token.as_str() {
                "true" => {
                    Ok(ASTNode::Boolean(true))
                }
                "false" => {
                    Ok(ASTNode::Boolean(false))
                }
                "EXPR" => {
                    let expr_node = gen_ast(&parse_node.children[0].clone().unwrap())?;
                    gen_booldash(&parse_node.children[1].clone().unwrap(), expr_node)
                }
                _ => Err(format!(
                    "Unidentified BOOL rule {:?}", parse_node.children[1].clone().unwrap()
                )),
            }
        }
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

fn gen_stmts(node: &ParseNode, inh_attr: ASTNode) -> Result<ASTNode, String> {
    if node.children.is_empty() || node.children[0].clone().unwrap().token == "EPSILON" {
        Ok(inh_attr)
    }
    else {
        let stmt_node = gen_ast(&node.children[0].clone().unwrap())?;
        let new_node = ASTNode::Stmts {
            stmt: Box::new(inh_attr),
            stmts: Box::new(stmt_node),
        };
        gen_stmts(&node.children[1].clone().unwrap(), new_node)
    }
}

fn gen_booldash(node: &ParseNode, inh_attr: ASTNode) -> Result<ASTNode, String> {
    match node.children[0].clone().unwrap().value.clone().unwrap().as_str() {
        "<" => {
            let expr_node = gen_ast(&node.children[1].clone().unwrap())?;
            Ok(ASTNode::Relop {
                opr: "<".to_string(),
                lhs: Box::new(inh_attr),
                rhs: Box::new(expr_node),
            })
        }
        "<=" => {
            let expr_node = gen_ast(&node.children[1].clone().unwrap())?;
            Ok(ASTNode::Relop {
                opr: "<=".to_string(),
                lhs: Box::new(inh_attr),
                rhs: Box::new(expr_node),
            })
        }
        ">" => {
            let expr_node = gen_ast(&node.children[1].clone().unwrap())?;
            Ok(ASTNode::Relop {
                opr: ">".to_string(),
                lhs: Box::new(inh_attr),
                rhs: Box::new(expr_node),
            })
        }
        ">=" => {
            let expr_node = gen_ast(&node.children[1].clone().unwrap())?;
            Ok(ASTNode::Relop {
                opr: ">=".to_string(),
                lhs: Box::new(inh_attr),
                rhs: Box::new(expr_node),
            })
        }
        "==" => {
            let expr_node = gen_ast(&node.children[1].clone().unwrap())?;
            Ok(ASTNode::Relop {
                opr: "==".to_string(),
                lhs: Box::new(inh_attr),
                rhs: Box::new(expr_node),
            })
        }
        _ => Err(format!(
            "Unidentified BOOLDASH rule {:?}", node.children[0].clone().unwrap()
        )),
    }
}

fn gen_exprdash(node: &ParseNode, inh_attr: ASTNode) -> Result<ASTNode, String> {
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

fn gen_termdash(node: &ParseNode, inh_attr: ASTNode) -> Result<ASTNode, String> {
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

// //TODO: need implementation
// pub fn bfs_ats(root: &ASTNode, level: usize, traversed: &mut HashMap<usize, Vec<String>>) {
//     match root {
//         ASTNode::Number(num) => {
//             traversed.entry(level).or_default().push(num.to_string());
//         }
//         ASTNode::Identifier(id) => {
//             traversed.entry(level).or_default().push(id.to_string());
//         }
//         ASTNode::Boolean(bool) => {
//             traversed.entry(level).or_default().push(bool.to_string());
//         }
//         ASTNode::Semicolon(semi) => {
//             traversed.entry(level).or_default().push(semi.to_string());
//         }
//         ASTNode::Prog{argdecl, typedecl, stmts, ret} => {
//             bfs_ats(&argdecl, level + 1, traversed);
//             bfs_ats(&typedecl, level + 1, traversed);
//             bfs_ats(&stmts, level + 1, traversed);
//             bfs_ats(&ret, level + 1, traversed);
//         }
//         ASTNode::Return{opr, var} => {
//             traversed.entry(level).or_default().push(opr.to_string());
//             traversed.entry(level).or_default().push(var.to_string());
//         }
//         ASTNode::ArgDecl{opr, var, tail} => {
//             traversed.entry(level).or_default().push(opr.to_string());
//             traversed.entry(level).or_default().push(var.to_string());
//             bfs_ats(&tail, level + 1, traversed);
//         }
//         ASTNode::TypeDecl{var_type, var, tail} => {
//             traversed.entry(level).or_default().push(var_type.to_string());
//             traversed.entry(level).or_default().push(var.to_string());
//             bfs_ats(&tail, level + 1, traversed);
//         }
//         ASTNode::Stmts{stmt, stmts} => {
//             traversed.entry(level).or_default().push(stmt.to_string());
//             bfs_ats(&stmts, level + 1, traversed);
//         }
//         ASTNode::Assign{var, val} => {
//             traversed.entry(level).or_default().push(var.to_string());
//             bfs_ats(&val, level + 1, traversed);
//         }
//         ASTNode::IfThenElse{cond, true_block, false_block} => {

//         }
//         ASTNode::Whileloop{cond, block} => {

//         }
//         ASTNode::Relop{opr, lhs, rhs} => {

//         }
//         ASTNode::Binop{opr, lhs, rhs} => {
//             traversed.entry(level).or_default().push(opr.to_string());
//             bfs_ats(&lhs, level + 1, traversed);
//             bfs_ats(&rhs, level + 1, traversed);
//         }
//         // _ => println!("No ASTNode"),
//     }
// }