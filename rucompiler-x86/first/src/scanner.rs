
pub fn is_terminal(sym: &str) -> bool {
    let terminals = ["+", "*", "(", ")", "IDENTIFIER", "NUMBER"];
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

pub fn separate_brackets(s: &str) -> String {
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