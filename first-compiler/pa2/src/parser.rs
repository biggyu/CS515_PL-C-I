use crate::scanner::*;
use crate::grammar::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub token: String,
    pub value: Option<String>,
    pub children: Vec<Result<ParseNode, String>>,
}

pub fn ll1_parse(expression: &mut Vec<String>, parse_table: &HashMap<(String, String), ProductionRule>, start_symbol: &str) -> Result<ParseNode, String> {
    // let mut input = expression.clone();
    expression.push("$".to_string());
    
    let mut parse_root = ParseNode{token: start_symbol.to_string(), value: None, children: Vec::new(),};
    // let mut parse_tree: HashMap<usize, Vec<String>> = HashMap::new();
    // parse_tree.entry(0).or_default().push(start_symbol.to_string());
    // let mut expr_index: usize = 0;
    let top = start_symbol.to_string();
    let cur_token = expression[0].clone();
    // let cur_token = &input[expr_index];
    if is_terminal(&top) || top == "$" {
        if *cur_token == top {
            parse_root.value = Some(cur_token.clone());
            expression.remove(0);
        }
        else if top == "IDENTIFIER" && is_identifier(&cur_token) {
            parse_root.value = Some(cur_token.clone());
            expression.remove(0);
        }
        else if top == "NUMBER" && is_number(&cur_token) {
            parse_root.value = Some(cur_token.clone());
            expression.remove(0);
        }
        else {
            return Err(format!("Syntax error: expected '{}', got '{}'", top, cur_token));
        }
    }
    else {
        let mut key = (String::new(), String::new());
        if is_number(&cur_token) {
            key = (top.to_string(), "NUMBER".to_string());
        }
        else if is_identifier(&cur_token) {
            key = (top.to_string(), "IDENTIFIER".to_string());
        }
        else {
            key = (top.to_string(), cur_token.to_string());
        }
        if let Some(rule) = parse_table.get(&key) {
            for (_, token) in tokenize(&rule.rhs).iter().enumerate() {
                if token != "EPSILON" {
                    let child = ll1_parse(expression, &parse_table, &token.to_string())?;
                    parse_root.children.push(Ok(child));
                }
                else {
                    parse_root.children.push(Ok(ParseNode{token: "EPSILON".to_string(), value: None, children: Vec::new(),}));
                }
            }
        }
        else {
            return Err(format!("No production rule for ({}, {}) in parse table", top, cur_token));
        }
    }
    // for action in actions {
    //     println!("{:?}", action);
    // }
    Ok(parse_root.clone())
}