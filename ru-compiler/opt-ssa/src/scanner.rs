
pub fn is_terminal(sym: &str) -> bool {
    let terminals = ["+", "*", "(", ")", "=", "{", "}", ">", "<", ">=", "<=", "==", "IDENTIFIER", "NUMBER", "return", "args", ";", "int", ",", "if", "then", "else", "while", "true", "false"];
    terminals.contains(&sym)
}

pub fn is_identifier(s: &str) -> bool {
    s.chars().all(|c| c.is_alphabetic())
}

pub fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

pub fn tokenize(rhs: &str) -> Vec<String> {
    rhs.split_whitespace().map(|s| s.to_string()).collect()
}

pub fn separate_chars(s: &str) -> Vec<String> {
    let mut sep = separate_brackets(&s);
    sep = separate_semicolons(&sep);
    sep = separate_commas(&sep);
    sep.split_whitespace().map(|s| s.to_string()).collect()
    
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

fn separate_semicolons(s: &str) -> String {
    let mut tokens = String::new();

    for ch in s.chars() {
        if ch == ';'{
            tokens.push(' ');
            tokens.push(ch);
        }
        else {
            tokens.push(ch);
        }
    }
    tokens
}

fn separate_commas(s: &str) -> String {
    let mut tokens = String::new();

    for ch in s.chars() {
        if ch == ','{
            tokens.push(' ');
            tokens.push(ch);
        }
        else {
            tokens.push(ch);
        }
    }
    tokens
}