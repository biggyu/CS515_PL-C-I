mod scanner;
mod grammar;
mod parser;
mod ast;
mod dag;
mod llvm;

use crate::scanner::*;
use crate::grammar::*;
use crate::parser::*;
use crate::ast::*;
use crate::dag::*;
use crate::llvm::*;

use std::env;
use std::fs;
use std::io;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let contents = fs::read_to_string(input_file).expect("Cannot read file");
    let lines = contents.lines();

    let mut expressions: Vec<_> = Vec::new();
    // let mut expressions: Vec<Vec<_>> = Vec::new();
    for line in lines {
        expressions.extend(separate_chars(&line));
    }
    // for expr in &expressions {
    //     println!("{}", expr);
    // }

    //Context Free Grammar
    let start_symbol = "PROG".to_string();
    let mut rules: Vec<ProductionRule> = Vec::new();
    rules.push(ProductionRule{lhs: String::from("PROG"), rhs: String::from("ARGDECL TYPEDECL STMTS RET")});
    rules.push(ProductionRule{lhs: String::from("RET"), rhs: String::from("return IDENTIFIER ;")});
    rules.push(ProductionRule{lhs: String::from("ARGDECL"), rhs: String::from("args IDENTIFIER ARGDECLTAIL")});
    rules.push(ProductionRule{lhs: String::from("ARGDECLTAIL"), rhs: String::from("; | IDENTIFIER ARGDECLTAIL")});
    rules.push(ProductionRule{lhs: String::from("TYPEDECL"), rhs: String::from("int IDENTIFIER TYPEDECLTAIL")});
    rules.push(ProductionRule{lhs: String::from("TYPEDECLTAIL"), rhs: String::from("; | , IDENTIFIER TYPEDECLTAIL")});
    rules.push(ProductionRule{lhs: String::from("STMTS"), rhs: String::from("STMT STMTS | EPSILON")});
    rules.push(ProductionRule{lhs: String::from("STMT"), rhs: String::from("ASSIGN | IFTHENELSE | WHILE")});
    rules.push(ProductionRule{lhs: String::from("ASSIGN"), rhs: String::from("IDENTIFIER = EXPR ;")});
    rules.push(ProductionRule{lhs: String::from("IFTHENELSE"), rhs: String::from("if BOOL then { STMTS } else { STMTS }")});
    rules.push(ProductionRule{lhs: String::from("WHILE"), rhs: String::from("while BOOL then { STMTS }")});
    rules.push(ProductionRule{lhs: String::from("BOOL"), rhs: String::from("true | false | EXPR <= EXPR | EXPR < EXPR | EXPR >= EXPR | EXPR > EXPR | EXPR == EXPR")});
    rules.push(ProductionRule{lhs: String::from("EXPR"), rhs: String::from("EXPR + TERM | TERM")});
    rules.push(ProductionRule{lhs: String::from("TERM"), rhs: String::from("TERM * FACTOR | FACTOR")});
    rules.push(ProductionRule{lhs: String::from("FACTOR"), rhs: String::from("IDENTIFIER | NUMBER | (EXPR)")});

    let llrules = elm_ambig(&mut rules);

    let mut firsts: HashMap<String, HashSet<String>> = HashMap::new();
    firsts = compute_first(&llrules);
    
    let mut follows: HashMap<String, HashSet<String>> = HashMap::new();
    follows = compute_follow(&llrules, &mut firsts, &start_symbol);

    let mut parse_table: HashMap<(String, String), ProductionRule> = HashMap::new();
    parse_table = gen_parse_table(&llrules, &firsts, &follows);
    // for key in parse_table.keys() {
    //     println!("{:?} {:?}", key, parse_table[key]);
    // }

    let parsed = ll1_parse(&mut expressions, &parse_table, &start_symbol);
    if let Ok(root) = parsed {
        // let mut ordered_keys = Vec::new();
        // println!("{:?}", root);
        // let mut parse_traversed = HashMap::new();
        // bfs_parse(&root, 0, &mut parse_traversed);
        // ordered_keys = parse_traversed.keys().into_iter().collect();
        // ordered_keys.sort_by(|x, y| x.cmp(&y));
        // for key in ordered_keys {
        //     for term in &parse_traversed[key] {
        //         print!("{} ", term);
        //     }
        //     println!();
        // }

        let ast = gen_ast(&root);
        if let Ok(root) = ast {
            // AST
            // let mut ast_traversed = HashMap::new();
            // bfs_ats(&root, 0, &mut ast_traversed);
            // ordered_keys = ast_traversed.keys().into_iter().collect();
            // ordered_keys.sort_by(|x, y| x.cmp(&y));
            // println!("AST");
            // for key in ordered_keys {
            //     for term in &ast_traversed[key] {
            //         print!("{} ", term);
            //     }
            //     println!();
            // }
            // println!("{:?}", root);
            
            // Directed Acyclic Representation
            let mut value_nums: HashMap<DAGNode, usize> = HashMap::new();
            let mut cur_valnum: usize = 0;
            let mut dag_nodes: HashMap<usize, Rc<DAGNode>> = HashMap::new();
            let (_, ast_dag) = dag_from_ast(&root, &mut value_nums, &mut cur_valnum, &mut dag_nodes);
            // println!("{:?}", ast_dag);
            // let mut ordered_keys: Vec<_> = ast_dag.keys().into_iter().collect();
            // ordered_keys.sort_by(|x, y| x.cmp(&y));
            // for key in ordered_keys {
            //     println!("{} {:?}", key, cfg_dags[key])
            // }
            
            // let mut dag_traversed = HashMap::new();
            // bfs_dag(&*dag.clone(), 0, &mut dag_traversed);
            // ordered_keys = dag_traversed.keys().into_iter().collect();
            // ordered_keys.sort_by(|x, y| x.cmp(&y));
            // println!("DAG");
            // for key in ordered_keys {
            //     for term in &dag_traversed[key] {
            //         print!("{} ", term);
            //     }
            //     println!();
            // }

            //LLVM IR
            let mut temp_nums: HashMap<DAGNode, usize> = HashMap::new();
            let llvm_ir = gen_llvm_ir(&*ast_dag.clone(), &mut temp_nums, &dag_nodes, Some("i64".to_string()), Some("foo".to_string()));
            let output_file: Vec<_> = input_file.split(".").collect();
            fs::write(format!("{}.ll", output_file[0]), &llvm_ir)?;
            // // println!("{}\nfirst.ll created successfully", llvm_ir);
        }
        else {
            println!("{:?}", ast);
        }
    }
    else {
        println!("{:?}", parsed);
    }

    // for expression in expressions {
    //     // parse tree
    //     let mut tmp = expression.iter().map(|s| s.to_string()).collect();
    //     let parsed = ll1_parse(&mut tmp, &parse_table, &start_symbol);
    //     //TODO: print format
    //     if let Ok(root) = parsed {
    //         let mut parse_traversed = HashMap::new();
    //         bfs_parse(&root, 0, &mut parse_traversed);
    //         let mut ordered_keys: Vec<_> = parse_traversed.keys().into_iter().collect();
    //         // ordered_keys.sort_by(|x, y| x.cmp(&y));
    //         // for key in ordered_keys {
    //         //     for term in &parse_traversed[key] {
    //         //         print!("{} ", term);
    //         //     }
    //         //     println!();
    //         // }

    //         let ast = gen_ast(&root);
    //         if let Ok(root) = ast {
    //             // AST
    //             let mut ast_traversed = HashMap::new();
    //             bfs_ats(&root, 0, &mut ast_traversed);
    //             ordered_keys = ast_traversed.keys().into_iter().collect();
    //             ordered_keys.sort_by(|x, y| x.cmp(&y));
    //             println!("AST");
    //             for key in ordered_keys {
    //                 for term in &ast_traversed[key] {
    //                     print!("{} ", term);
    //                 }
    //                 println!();
    //             }
    //             // Directed Acyclic Representation
    //             let mut value_nums: HashMap<DAGNode, usize> = HashMap::new();
    //             let mut cur_valnum: usize = 0;
    //             let mut dag_nodes: HashMap<usize, Rc<DAGNode>> = HashMap::new();
    //             let (_, dag) = dag_rep(&root, &mut value_nums, &mut cur_valnum, &mut dag_nodes);

    //             let mut dag_traversed = HashMap::new();
    //             bfs_dag(&*dag.clone(), 0, &mut dag_traversed);
    //             ordered_keys = dag_traversed.keys().into_iter().collect();
    //             ordered_keys.sort_by(|x, y| x.cmp(&y));
    //             println!("DAG");
    //             for key in ordered_keys {
    //                 for term in &dag_traversed[key] {
    //                     print!("{} ", term);
    //                 }
    //                 println!();
    //             }

    //             //LLVM IR
    //             let mut temp_nums: HashMap<DAGNode, usize> = HashMap::new();
    //             let llvm_ir = gen_llvm_ir(&*dag.clone(), &mut temp_nums, &dag_nodes, Some("i64".to_string()), Some("foo".to_string()));
    //             let output_file: Vec<_> = input_file.split(".").collect();
    //             fs::write(format!("{}.ll", output_file[0]), &llvm_ir)?;
    //             // println!("{}\nfirst.ll created successfully", llvm_ir);
    //         }
    //         else {
    //             println!("{:?}", ast);
    //         }
    //     }
    //     else {
    //         println!("{:?}", parsed);
    //     }
    //     println!();
    // }
    Ok(())
}