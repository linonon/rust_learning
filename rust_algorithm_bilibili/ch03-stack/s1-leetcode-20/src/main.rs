use ds_lib::stack;

fn main() {
    assert_eq!(is_valid_quote("{}[]()"), true);
    assert_eq!(is_valid_quote("{{}}[]()"), true);
    assert_eq!(is_valid_quote("{{}}[9()"), false);
}

// 括号匹配
fn is_valid_quote(quote: &str) -> bool {
    let mut s = stack::Stack::new(quote.len());
    for c in quote.chars() {
        if c == '(' || c == '[' || c == '{' {
            s.push(c);
            continue;
        } else {
            let top = s.pop();
            if top == '(' && c == ')' {
                continue;
            } else if top == '[' && c == ']' {
                continue;
            } else if top == '{' && c == '}' {
                continue;
            } else {
                return false;
            }
        }
    }

    if !s.empty() {
        return false;
    }

    true
}
