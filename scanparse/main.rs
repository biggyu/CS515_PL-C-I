use std::env;
use std::fs;

//TODO: Terminal, NonTerminal ?
// struct NonTerminal {
//     token_type: String,

// }

// struct Terminal {
//     token_type: String,
//     value: String,
// }

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

    let mut rules: Vec<ProductionRule> = Vec::new();
    rules.push(ProductionRule{lhs: "EXPR".to_string(), rhs: "TERM EXPRDASH".to_string()});
    rules.push(ProductionRule{lhs: "EXPRDASH".to_string(), rhs: "+ TERM EXPRDASH".to_string()});
    rules.push(ProductionRule{lhs: "EXPRDASH".to_string(), rhs: "EPSILON".to_string()});
    rules.push(ProductionRule{lhs: "TERM".to_string(), rhs: "FACTOR TERMDASH".to_string()});
    rules.push(ProductionRule{lhs: "TERMDASH".to_string(), rhs: "* FACTOR TERMDASH".to_string()});
    rules.push(ProductionRule{lhs: "TERMDASH".to_string(), rhs: "EPSILON".to_string()});
    rules.push(ProductionRule{lhs: "FACTOR".to_string(), rhs: "IDENTIFIER".to_string()});
    rules.push(ProductionRule{lhs: "FACTOR".to_string(), rhs: "NUMBER".to_string()});
    rules.push(ProductionRule{lhs: "FACTOR".to_string(), rhs: "(EXPR)".to_string()});
    
    for rule in rules {
        println!("{} -> {}", rule.lhs, rule.rhs);
    }

    // let tmp_parse: i32 = "0123".parse().unwrap();
    // println!("{}", tmp_parse);

}
fn scanner() {

}

fn parser() {

}

// fn elm_ambig(rules: &mut Vec<ProductionRule>) -> Vec<ProductionRule> {

// }

// fn elm_leftRecursion(rules: &mut Vec<ProductionRule>) -> Vec<ProductionRule> {

// }

// fn elm_leftFactoring(rules: &mut Vec<ProductionRule>) -> Vec<ProductionRule> {

// }