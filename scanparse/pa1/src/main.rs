use std::env;
use std::fs;
use std::collections::{HashMap, HashSet};

const PLUS: &str = "+";
const STAR: &str = "*";
const BOPEN: &str = "(";
const BCLOSE: &str = ")";

enum Token {

}

#[derive(Clone, Debug)]
struct ProductionRule {
    // lhs: NonTerminal,
    // rhs: String,
    lhs: String,
    rhs: String,
}

// #[derive(Debug, Clone)]
// struct ParseNode {
//     token: String,
//     value: Option<String>,
//     children: Vec<ParseNode>,
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let contents = fs::read_to_string(input_file).expect("Cannot read file");
    let mut lines = contents.lines();

    let mut expressions: Vec<Vec<_>> = Vec::new();
    
    for line in lines {
        expressions.push(line.split_whitespace().collect());
        for expr in expressions.last_mut() {
            let mut i = 0;
            while i < expr.len() {
                if let Some(index) = expr[i].find(&BOPEN) {
                    *expr = [&expr[0..i], &[BOPEN], &[&expr[i][(index + 1)..]], &expr[(i + 1)..]].concat();
                }
                if let Some(index) = expr[i].find(&BCLOSE) {
                    if index + 1 == expr[i].len() {
                        *expr = [&expr[0..i], &[&expr[i][..index]], &[BCLOSE], &expr[(i + 1)..]].concat();
                    }
                    else if index != 0{
                        *expr = [&expr[0..i], &[&expr[i][..index]], &[BCLOSE], &[&expr[i][(index+1)..]], &expr[(i + 1)..]].concat();
                        // println!("{:?}\n", &expr);
                    }
                    else {
                        *expr = [&expr[0..i], &[BCLOSE], &[&expr[i][(index+1)..]], &expr[(i + 1)..]].concat();
                    }
                    i += 1;
                    // println!("{}", expr.len());
                }
                i += 1;
            }
        }
    }
    // for expr in &expressions {
    //     println!("{:?}", expr);
    // }
    let start_symbol = "EXPR".to_string();
    let mut rules: Vec<ProductionRule> = Vec::new();
    rules.push(ProductionRule{lhs: String::from("EXPR"), rhs: String::from("EXPR + TERM | TERM")});
    rules.push(ProductionRule{lhs: String::from("TERM"), rhs: String::from("TERM * FACTOR | FACTOR")});
    rules.push(ProductionRule{lhs: String::from("FACTOR"), rhs: String::from("IDENTIFIER | NUMBER | (EXPR)")});

    let mut llrules = elm_ambig(&mut rules);

    let mut firsts: HashMap<String, HashSet<String>> = HashMap::new();
    firsts = compute_first(&llrules);
    
    let mut follows: HashMap<String, HashSet<String>> = HashMap::new();
    follows = compute_follow(&llrules, &mut firsts, &start_symbol);

    let mut parse_table: HashMap<(String, String), ProductionRule> = HashMap::new();
    parse_table = gen_parse_table(&llrules, &firsts, &follows);

    for expression in expressions {
        let tmp = expression.iter().map(|s| s.to_string()).collect();
        let parsed = ll1_parse(&tmp, &parse_table, &start_symbol);
        if let Ok(root) = parsed {
            let mut ordered_keys: Vec<_> = root.keys().into_iter().collect();
            ordered_keys.sort_by(|x, y| x.cmp(&y));
            for key in ordered_keys {
                for term in &root[key] {
                    print!("{} ", term);
                }
                println!();
            }
        }
        else {
            println!("{:?}", parsed);
        }
        println!();
    }
}
// fn scanner() {

// }

// fn parser() {

// }

fn elm_ambig(rules: &mut Vec<ProductionRule>) -> Vec<ProductionRule> {
    let mut llrules = elm_leftRecursion(rules);
    // llrules = elm_leftFactoring(&mut llrules);
    for mut rule in &mut llrules {
        for token in tokenize(&rule.rhs) {
            let mut tmp: String = token.clone();
            if tmp.contains(&BOPEN) || tmp.contains(&BCLOSE) {
                rule.rhs = separate_brackets(&tmp);
            }
        }
    }
    
    llrules
}

fn elm_leftRecursion(rules: &mut Vec<ProductionRule>) -> Vec<ProductionRule> {
    let mut rec_rules: Vec<ProductionRule> = Vec::new();
    for rule in rules {
        let mut rule_rhs = rule.rhs.split("|");
        let mut check: Vec<bool> = Vec::new();
        for (idx, elm) in rule_rhs.clone().enumerate() {
            let tmp: Vec<_> = elm.split_whitespace().collect();
            check.push(rule.lhs == tmp[0]);
        }
        if check.iter().any(|&b| b) {
            for (idx, elm) in rule_rhs.clone().enumerate() {
                if check[idx] {
                    let tmp1: Vec<_> = elm.split_whitespace().collect();
                    let tmp2: String = tmp1[1..].iter().map(|i| i.to_string() + " ").collect();
                    rec_rules.push(ProductionRule{lhs: rule.lhs.clone() + "DASH", rhs: tmp2 + &rule.lhs.clone() + "DASH"});
                    rec_rules.push(ProductionRule{lhs: rule.lhs.clone() + "DASH", rhs: String::from("EPSILON")});   
                }
                else {
                    rec_rules.push(ProductionRule{lhs: rule.lhs.clone(), rhs: String::from(elm) + " " + &rule.lhs.clone() + "DASH"});
                }
            }
        }
        else {
            for (idx, elm) in rule_rhs.clone().enumerate() {
                rec_rules.push(ProductionRule{lhs: rule.lhs.clone(), rhs: String::from(elm)});
            }
        }
    }
    // for rule in &rec_rules {
    //     println!("{} -> {}", rule.lhs, rule.rhs);
    // }
    rec_rules
}

//TODO: need implementation
fn elm_leftFactoring(rules: &mut Vec<ProductionRule>) -> Vec<ProductionRule> {
    let mut fac_rules: Vec<ProductionRule> = Vec::new();
    for rule in rules {
        let mut rule_rhs = rule.rhs.split("|");
        // println!("{:?}", rule_rhs);
        let mut alphas = Vec::new();
        for elm in rule_rhs {
            let tmp: Vec<_> = elm.split_whitespace().collect();
            alphas.push(tmp[0]);
        }
        // println!("alphas: {:?}", alphas);
    }
    fac_rules
}

fn compute_first(llrules: &Vec<ProductionRule>) -> HashMap<String, HashSet<String>> {
    let mut first: HashMap<String, HashSet<String>> = HashMap::new();
    for rule in llrules {
        for token in &tokenize(&rule.rhs) {
            if is_terminal(token) {
                first.entry(token.to_string()).or_default().insert(token.to_string());
            }
        }
    }
    first.entry("EPSILON".to_string()).or_default().insert("EPSILON".to_string());
    let mut changed = true;
    while changed {
        changed = false;
        for rule in llrules {
            let mut eps_all = true;
            let mut len: usize = 0;
            for token in &tokenize(&rule.rhs) {
                let tok_first = first.get(token).cloned().unwrap_or_default();
                let lhs_first = first.entry(rule.lhs.clone()).or_default();

                len = lhs_first.len();
    
                lhs_first.extend(tok_first.iter().filter(|&t| t != "EPSILON").map(|s| s.to_string()));
                if !tok_first.contains("EPSILON") {
                    eps_all = false;
                    break;
                }
            }
            if eps_all {
                first.entry(rule.lhs.clone()).or_default().insert("EPSILON".to_string());
            }
            if first[&rule.lhs].len() > len {
                changed = true;
            }
        }
        // for key in first.keys() {
        //     println!("{}->{:?}", key, first[key].iter());
        // }
        // println!("");
    }
    first.remove("EPSILON");
    // for key in first.keys() {
    //     println!("{}->{:?}", key, first[key].iter());
    // }
    first
}

fn compute_follow(llrules: &Vec<ProductionRule>, firsts: &mut HashMap<String, HashSet<String>>, start_symbol: &str) -> HashMap<String, HashSet<String>> {
    let mut follow: HashMap<String, HashSet<String>> = HashMap::new();
    follow.entry(start_symbol.to_string()).or_default().insert("$".to_string());

    let mut changed = true;
    while changed {
        changed = false;
        for rule in llrules {
            let rhs = tokenize(&rule.rhs);

            let A = &rule.lhs;
            let mut len: usize = 0;
            if rhs.len() > 1 {
                let A_follow = follow.get(A).cloned().unwrap_or_default();
                let B_follow = follow.entry(rhs[1].clone()).or_default();
    
                len = B_follow.len();
                
                if rhs.len() == 2 {
                    B_follow.extend(A_follow.iter().map(|s| s.to_string()));
                }
                else if rhs.len() == 3 {
                    let beta_first = firsts.entry(rhs[2].clone()).or_default();
                    B_follow.extend(beta_first.iter().filter(|&t| t != "EPSILON").map(|s| s.to_string()));
                    if beta_first.contains("EPSILON") {
                        B_follow.extend(A_follow.iter().map(|s| s.to_string()));
                    }
                }
                if B_follow.len() > len {
                    changed = true;
                }
            }
        }
        // for key in follow.keys() {
        //     println!("{}->{:?}", key, follow[key].iter());
        // }
        // println!("");
    }
    // for key in follow.keys() {
    //     println!("{}->{:?}", key, follow[key].iter());
    // }

    follow
}

fn gen_parse_table(llrules: &Vec<ProductionRule>, firsts: &HashMap<String, HashSet<String>>, follows: &HashMap<String, HashSet<String>>) -> HashMap<(String, String), ProductionRule> {
    let mut parse_table: HashMap<(String, String), ProductionRule> = HashMap::new();

    for rule in llrules {
        let rhs_token = tokenize(&rule.rhs);
        let mut rhs_first = HashSet::new();
        let mut eps_all = true;
        for token in rhs_token {
            let tok_first = firsts.get(&token).cloned().unwrap_or(HashSet::from([token.to_string()]));

            rhs_first.extend(tok_first.iter().filter(|&t| t != "EPSILON").map(|s| s.to_string()));
            if !tok_first.contains("EPSILON") {
                eps_all = false;
                break;
            }
        }
        if eps_all {
            rhs_first.insert("EPSILON".to_string());
        }

        for term in rhs_first.iter().filter(|&s| s != "EPSILON") {
            parse_table.insert((rule.lhs.clone(), term.clone()), ProductionRule{lhs: rule.lhs.clone(), rhs: rule.rhs.clone()});
        }

        if rhs_first.contains("EPSILON") {
            if let Some(follow) = follows.get(&rule.lhs) {
                for term in follow {
                    parse_table.insert((rule.lhs.clone(), term.clone()), ProductionRule{lhs: rule.lhs.clone(), rhs: rule.rhs.clone()});
                }
            }
        }
    }
    // for nterm in parse_table.keys() {
    //     println!("{:?} {:?}\n", nterm, parse_table[nterm]);
    // }
    
    parse_table
}

fn ll1_parse(expression: &Vec<String>, parse_table: &HashMap<(String, String), ProductionRule>, start_symbol: &str) -> Result<HashMap<usize, Vec<String>>, String> {
    let mut input = expression.clone();
    input.push("$".to_string());
    
    // let mut root = ParseNode{token: start_symbol.to_string(), value: None, children: Vec::new()};
    // let mut stack: Vec<(ParseNode, Vec<String>)> = vec![(root.clone(), vec![start_symbol.to_string()])];
    let mut stack: Vec<(String, usize)> = vec![("$".to_string(), 0), (start_symbol.to_string(), 0)];
    let mut actions: Vec<(String, usize)> = Vec::new();
    let mut parse_tree: HashMap<usize, Vec<String>> = HashMap::new();
    parse_tree.entry(0).or_default().push(start_symbol.to_string());
    let mut expr_index: usize = 0;

    while let Some((top, level)) = stack.pop() {
        let cur_token = &input[expr_index];
        if is_terminal(&top) || top == "$" {
            if *cur_token == top {
                actions.push((format!("Matched {}", cur_token).to_string(), level));
                expr_index += 1;
            }
            else if top == "IDENTIFIER" && is_identifier(&cur_token) {
                actions.push((format!("Matched IDENTIFIER({})", cur_token).to_string(), level));
                expr_index += 1;
            }
            else if top == "NUMBER" && is_number(&cur_token) {
                actions.push((format!("Matched NUMBER({})", cur_token).to_string(), level));
                expr_index += 1;
            }
            else {
                return Err(format!(
                    "Syntax error: expected '{}', got '{}'",
                    top, cur_token
                ));
            }
        }
        else {
            let mut key = ("".to_string(), "".to_string());
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
                let rhs_token = tokenize(&rule.rhs);
                for token in tokenize(&rule.rhs).iter().rev() {
                    if token != "EPSILON"{
                        stack.push((token.to_string(), level + 1));
                    }
                }
                for token in tokenize(&rule.rhs) {
                    if is_terminal(&token) && cur_token == STAR {
                        parse_tree.entry(level + 1).or_default().push("STAR".to_string());
                    }
                    else if is_terminal(&token) && cur_token == PLUS {
                        parse_tree.entry(level + 1).or_default().push("PLUS".to_string());
                    }
                    else if token == "IDENTIFIER" && is_identifier(&cur_token) {
                        parse_tree.entry(level + 1).or_default().push(format!("IDENTIFIER({})", cur_token.to_string()));
                    }
                    else if token == "NUMBER" && is_number(&cur_token) {
                        parse_tree.entry(level + 1).or_default().push(format!("NUMBER({})", cur_token.to_string()));
                    }
                    else {
                        parse_tree.entry(level + 1).or_default().push(token.to_string());
                    }
                }
                actions.push((format!("Apply {:?}", rule).to_string(), level));
            }
            else {
                return Err(format!(
                    "No production rule for ({}, {}) in parse table",
                    top, cur_token
                ));
            }
        }
    }
    // for action in actions {
    //     println!("{:?}", action);
    // }
    Ok(parse_tree.clone())
}

fn is_terminal(sym: &str) -> bool {
    let terminals = ["+", "*", "(", ")", "IDENTIFIER", "NUMBER"];
    terminals.contains(&sym)
}

fn is_identifier(s: &str) -> bool {
    s.chars().all(|c| c.is_alphabetic())
}

fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

fn tokenize(rhs: &str) -> Vec<String> {
    rhs.split_whitespace().map(|s| s.to_string()).collect()
}

fn separate_brackets(s: &str) -> String {
    let mut tokens = String::new();

    for ch in s.chars() {
        if ch == '('{
            tokens.push(ch);
            tokens.push(' ');
        }
        else if ch == ')' {
            tokens.push(' ');
            tokens.push(ch);
        }
        else {
            tokens.push(ch);
        }
    }
    tokens
}