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

    let mut expressions: Vec<Vec<_>> = Vec::new();
    for line in lines {
        let separated = separate_brackets(&line);
        let tokens: Vec<String> = separated.split_whitespace().map(|s| s.to_string()).collect();

        expressions.push(tokens);
    }
    // for expr in expressions {
    //     println!("{:?}", expr);
    // }

    //Context Free Grammar
    let start_symbol = "EXPR".to_string();
    let mut rules: Vec<ProductionRule> = Vec::new();
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

    for expression in expressions {
        // parse tree
        let mut tmp = expression.iter().map(|s| s.to_string()).collect();
        let parsed = ll1_parse(&mut tmp, &parse_table, &start_symbol);
        //TODO: print format
        if let Ok(root) = parsed {
            let mut parse_traversed = HashMap::new();
            bfs_parse(&root, 0, &mut parse_traversed);
            let mut ordered_keys: Vec<_> = parse_traversed.keys().into_iter().collect();
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
                let mut ast_traversed = HashMap::new();
                bfs_ats(&root, 0, &mut ast_traversed);
                ordered_keys = ast_traversed.keys().into_iter().collect();
                ordered_keys.sort_by(|x, y| x.cmp(&y));
                println!("AST");
                for key in ordered_keys {
                    for term in &ast_traversed[key] {
                        print!("{} ", term);
                    }
                    println!();
                }
                // Directed Acyclic Representation
                let mut value_nums: HashMap<DAGNode, usize> = HashMap::new();
                let mut cur_valnum: usize = 0;
                let mut dag_nodes: HashMap<usize, Rc<DAGNode>> = HashMap::new();
                let (_, dag) = dag_rep(&root, &mut value_nums, &mut cur_valnum, &mut dag_nodes);

                let mut dag_traversed = HashMap::new();
                bfs_dag(&*dag.clone(), 0, &mut dag_traversed);
                ordered_keys = dag_traversed.keys().into_iter().collect();
                ordered_keys.sort_by(|x, y| x.cmp(&y));
                println!("DAG");
                for key in ordered_keys {
                    for term in &dag_traversed[key] {
                        print!("{} ", term);
                    }
                    println!();
                }

                //LLVM IR
                let mut temp_nums: HashMap<DAGNode, usize> = HashMap::new();
                let llvm_ir = gen_llvm_ir(&*dag.clone(), &mut temp_nums, &dag_nodes, Some("i64".to_string()), Some("foo".to_string()));
                let output_file: Vec<_> = input_file.split(".").collect();
                fs::write(format!("{}.ll", output_file[0]), &llvm_ir)?;
                // println!("{}\nfirst.ll created successfully", llvm_ir);
            }
            else {
                println!("{:?}", ast);
            }
        }
        else {
            println!("{:?}", parsed);
        }
        println!();
    }
    Ok(())
}