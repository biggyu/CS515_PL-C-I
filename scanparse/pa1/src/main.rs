use std::env;
use std::fs;
use std::collections::{HashMap, HashSet};

const PLUS: &str = "+";
const STAR: &str = "*";
const BOPEN: &str = "(";
const BCLOSE: &str = ")";

//TODO: Terminal, NonTerminal ?
// struct NonTerminal {
//     token_type: String,

// }

// struct Terminal {
//     token_type: String,
//     value: String,
// }

enum Token {

}

#[derive(Clone, Debug)]
struct ProductionRule {
    // lhs: NonTerminal,
    // rhs: String,
    lhs: String,
    rhs: String,
}

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
    
    let mut rules: Vec<ProductionRule> = Vec::new();
    rules.push(ProductionRule{lhs: String::from("EXPR"), rhs: String::from("EXPR + TERM | TERM")});
    rules.push(ProductionRule{lhs: String::from("TERM"), rhs: String::from("TERM * FACTOR | FACTOR")});
    rules.push(ProductionRule{lhs: String::from("FACTOR"), rhs: String::from("IDENTIFIER | NUMBER | (EXPR)")});

    let mut llrules = elm_ambig(&mut rules);
    // println!("llrules");
    // for llrule in &llrules {
    //     println!("{} -> {}", llrule.lhs, llrule.rhs);
    // }
    // println!("");
    let mut firsts: HashMap<String, HashSet<String>> = HashMap::new();
    firsts = compute_first(&llrules);
    // println!("first");
    // for key in firsts.keys() {
    //     println!("{} -> {:?}", key, firsts[key].iter());
    // }
    // println!("");
    let mut follows: HashMap<String, HashSet<String>> = HashMap::new();
    follows = compute_follow(&llrules, &mut firsts, "EXPR");
    // println!("follow");
    // for key in follows.keys() {
    //     println!("{} -> {:?}", key, follows[key].iter());
    // }
    // println!("");

    //TODO: construct first, follow, parse table ??
    // let mut parse_table: HashMap<String, HashMap<String, ProductionRule>> = HashMap::new();
    // parse_table.insert("EXPR".to_string(), HashMap::new());
    // parse_table.insert("EXPRDASH".to_string(), HashMap::new());
    // parse_table.insert("TERM".to_string(), HashMap::new());
    // parse_table.insert("TERMDASH".to_string(), HashMap::new());
    // parse_table.insert("FACTOR".to_string(), HashMap::new());

    // parse_table.get_mut("EXPR").unwrap().insert("IDENTIFIER".to_string(), ProductionRule{lhs: rules[0].lhs.clone(), rhs: rules[0].rhs.clone()});
    // parse_table.get_mut("EXPR").unwrap().insert("NUMBER".to_string(), ProductionRule{lhs: rules[0].lhs.clone(), rhs: rules[0].rhs.clone()});
    // parse_table.get_mut("EXPR").unwrap().insert("(".to_string(), ProductionRule{lhs: rules[0].lhs.clone(), rhs: rules[0].rhs.clone()});
    // parse_table.get_mut("EXPRDASH").unwrap().insert("+".to_string(), ProductionRule{lhs: rules[1].lhs.clone(), rhs: rules[1].rhs.clone()});
    // parse_table.get_mut("EXPRDASH").unwrap().insert(")".to_string(), ProductionRule{lhs: rules[2].lhs.clone(), rhs: rules[2].rhs.clone()});
    // parse_table.get_mut("EXPRDASH").unwrap().insert("$".to_string(), ProductionRule{lhs: rules[2].lhs.clone(), rhs: rules[2].rhs.clone()});
    // parse_table.get_mut("TERM").unwrap().insert("IDENTIFIER".to_string(), ProductionRule{lhs: rules[3].lhs.clone(), rhs: rules[3].rhs.clone()});
    // parse_table.get_mut("TERM").unwrap().insert("NUMBER".to_string(), ProductionRule{lhs: rules[3].lhs.clone(), rhs: rules[3].rhs.clone()});
    // parse_table.get_mut("TERM").unwrap().insert("(".to_string(), ProductionRule{lhs: rules[3].lhs.clone(), rhs: rules[3].rhs.clone()});
    // parse_table.get_mut("EXPRDASH").unwrap().insert("+".to_string(), ProductionRule{lhs: rules[5].lhs.clone(), rhs: rules[5].rhs.clone()});
    // parse_table.get_mut("EXPRDASH").unwrap().insert("*".to_string(), ProductionRule{lhs: rules[4].lhs.clone(), rhs: rules[4].rhs.clone()});
    // parse_table.get_mut("EXPRDASH").unwrap().insert(")".to_string(), ProductionRule{lhs: rules[5].lhs.clone(), rhs: rules[5].rhs.clone()});
    // parse_table.get_mut("EXPRDASH").unwrap().insert("$".to_string(), ProductionRule{lhs: rules[5].lhs.clone(), rhs: rules[2].rhs.clone()});
    // parse_table.get_mut("FACTOR").unwrap().insert("IDENTIFIER".to_string(), ProductionRule{lhs: rules[6].lhs.clone(), rhs: rules[6].rhs.clone()});
    // parse_table.get_mut("FACTOR").unwrap().insert("NUMBER".to_string(), ProductionRule{lhs: rules[7].lhs.clone(), rhs: rules[7].rhs.clone()});
    // parse_table.get_mut("FACTOR").unwrap().insert("(".to_string(), ProductionRule{lhs: rules[8].lhs.clone(), rhs: rules[8].rhs.clone()});

    // let mut firsts: HashMap<String, Vec<String>> = HashMap::new();
    // firsts.insert("EXPR".to_string(), Vec::new());
    // firsts.insert("EXPRDASH".to_string(), Vec::new());
    // firsts.insert("TERM".to_string(), Vec::new());
    // firsts.insert("TERMDASH".to_string(), Vec::new());
    // firsts.insert("FACTOR".to_string(), Vec::new());

    // firsts.get_mut("EXPR").unwrap().push("IDENTIFIER".to_string());
    // firsts.get_mut("EXPR").unwrap().push("(".to_string());
    // firsts.get_mut("TERM").unwrap().push("IDENTIFIER".to_string());
    // firsts.get_mut("TERM").unwrap().push("(".to_string());
    // firsts.get_mut("FACTOR").unwrap().push("IDENTIFIER".to_string());
    // firsts.get_mut("FACTOR").unwrap().push("(".to_string());
    // firsts.get_mut("EXPRDASH").unwrap().push("+".to_string());
    // firsts.get_mut("EXPRDASH").unwrap().push("EPSILON".to_string());
    // firsts.get_mut("TERMDASH").unwrap().push("*".to_string());
    // firsts.get_mut("TERMDASH").unwrap().push("EPSILON".to_string());

    // for rule in rules {
    //     println!("{} -> {}", rule.lhs, rule.rhs);
    // }

    // for expr in expressions {
    //     println!("{:?}", expr);
    // }

    // let tmp_parse: i32 = "0123".parse().unwrap();
    // println!("{}", tmp_parse);

}
fn scanner() {

}

fn parser() {

}

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

fn is_terminal(sym: &str) -> bool {
    let terminals = ["+", "*", "(", ")", "IDENTIFIER", "NUMBER"];
    terminals.contains(&sym)
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