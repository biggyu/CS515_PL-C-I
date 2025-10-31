use crate::scanner::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct ProductionRule {
    pub lhs: String,
    pub rhs: String,
}

pub fn elm_ambig(rules: &mut Vec<ProductionRule>) -> Vec<ProductionRule> {
    let mut llrules = elm_leftRecursion(rules);
    llrules = elm_leftFactoring(&mut llrules);
    for rule in &mut llrules {
        for token in tokenize(&rule.rhs) {
            let tmp: String = token.clone();
            if tmp.contains(&"(") || tmp.contains(&")") {
                rule.rhs = separate_brackets(&tmp);
            }
        }
    }
    
    llrules
}

pub fn elm_leftRecursion(rules: &mut Vec<ProductionRule>) -> Vec<ProductionRule> {
    let mut rec_rules: Vec<ProductionRule> = Vec::new();
    for rule in rules {
        let rule_rhs = rule.rhs.split("|");
        let mut check: Vec<bool> = Vec::new();
        for (_, elm) in rule_rhs.clone().enumerate() {
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
            for (_, elm) in rule_rhs.clone().enumerate() {
                rec_rules.push(ProductionRule{lhs: rule.lhs.clone(), rhs: String::from(elm)});
            }
        }
    }
    // for rule in &rec_rules {
    //     println!("{} -> {}", rule.lhs, rule.rhs);
    // }
    rec_rules
}

pub fn elm_leftFactoring(rules: &mut Vec<ProductionRule>) -> Vec<ProductionRule> {
    let mut fac_rules: Vec<ProductionRule> = Vec::new();
    let mut lhs = String::new();
    let mut rhs_alpha = String::new();
    let mut alphas = Vec::new();
    for rule in &mut *rules {
        let alpha = tokenize(&rule.rhs)[0].clone();
        if lhs == rule.lhs.to_string() && rhs_alpha == alpha {
            if !alphas.contains(&(lhs.to_string(), alpha.to_string())) {
                fac_rules.push(ProductionRule{lhs: rule.lhs.to_string(), rhs: alpha.clone() + " " + &rule.lhs.to_string() + "DASH"});
                alphas.push((lhs.to_string(), alpha.to_string()));
            }
            let tmp1: Vec<_> = rule.rhs.split_whitespace().collect();
            let tmp2: String = tmp1[1..].iter().map(|i| i.to_string() + " ").collect();
            fac_rules.push(ProductionRule{lhs: rule.lhs.to_string() + "DASH", rhs: tmp2});
        }
        else {
            fac_rules.push(ProductionRule{lhs: rule.lhs.to_string(), rhs: rule.rhs.to_string()});
            lhs = rule.lhs.to_string();
            rhs_alpha = alpha;
        }
    }
    for alpha in alphas {
        for mut rule in &mut fac_rules {
            let rhs_alpha = tokenize(&rule.rhs)[0].clone();
            if (rule.lhs.clone(), rhs_alpha.clone()) == alpha {
                let tmp1: Vec<_> = rule.rhs.split_whitespace().collect();
                let tmp2: String = tmp1[1..].iter().map(|i| i.to_string() + " ").collect();
                rule.lhs = rule.lhs.to_string() + "DASH";
                rule.rhs = tmp2;
                break;
            }
        }
    }
    // for rule in &fac_rules {
    //     println!("{} -> {}", rule.lhs, rule.rhs);
    // }
    fac_rules
}

pub fn compute_first(llrules: &Vec<ProductionRule>) -> HashMap<String, HashSet<String>> {
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

pub fn compute_follow(llrules: &Vec<ProductionRule>, firsts: &mut HashMap<String, HashSet<String>>, start_symbol: &str) -> HashMap<String, HashSet<String>> {
    let mut follow: HashMap<String, HashSet<String>> = HashMap::new();
    follow.entry(start_symbol.to_string()).or_default().insert("$".to_string());

    let mut changed = true;
    while changed {
        changed = false;
        for rule in llrules {
            let rhs = tokenize(&rule.rhs);

            let a = &rule.lhs;
            let mut len: usize = 0;

            let a_follow = follow.get(a).cloned().unwrap_or_default();
            for mut i in 0..=rhs.len() - 1 {
                if !is_terminal(&rhs[i]) {
                    let b_follow = follow.entry(rhs[i].clone()).or_default();
                    len = b_follow.len();
                    if i < rhs.len() - 1 {
                        // let mut has_epsilon = true;
                        let mut j = 1;
                        while true {
                            if i + j <= rhs.len() - 1 {
                                let beta_first = firsts.entry(rhs[i + j].clone()).or_default();
                                if beta_first.contains("EPSILON") {
                                    b_follow.extend(beta_first.iter().filter(|&t| t != "EPSILON").map(|s| s.to_string()));
                                    j += 1;
                                }
                                else {
                                    b_follow.extend(beta_first.iter().map(|s| s.to_string()));
                                    break;
                                }
                            }
                            else {
                                b_follow.extend(a_follow.iter().filter(|&t| t != "EPSILON").map(|s| s.to_string()));
                                i += j;
                                break;
                            }
                        }
                    }
                    else {
                        b_follow.extend(a_follow.iter().map(|s| s.to_string()));
                    }
                    if b_follow.len() > len {
                        changed = true;
                    }
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

pub fn gen_parse_table(llrules: &Vec<ProductionRule>, firsts: &HashMap<String, HashSet<String>>, follows: &HashMap<String, HashSet<String>>) -> HashMap<(String, String), ProductionRule> {
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