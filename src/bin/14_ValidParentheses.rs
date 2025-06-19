fn main() {
    let res = solve_fun(String::from("[()[]{}]"));
    println!("{:?}", res)
}

fn solve_fun(s: String) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    let mut stack: Vec<char> = vec![];

    for c in chars {

        if c == '{' || c == '[' || c == '(' {
            stack.push(c);
            continue;
        };
        
        let last_char = match stack.pop() {
            Some(c) => c,
            None => return false
        };

        if c == '}' && last_char != '{' {
            return false;
        } else if c == ']' && last_char != '[' {
            return false;
        } else if c == ')' && last_char != '(' {
            return false;
        };

    }
    return stack.len() == 0;
}
