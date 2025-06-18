fn main() {
    let res = solve_fun_optomized_better_looking(String::from("A man, a plan, a canal: Panama"));
    println!("{:?}", res)
}


fn solve_fun_optomized_better_looking(s: String) -> bool {
    let s: Vec<char> = s.chars().filter_map(|c| {
        if c.is_alphanumeric(){
            Some(c.to_ascii_lowercase())
        }
        else{
            None
        }
    }).collect();
    let mut left = 0;
    let mut right = s.len().saturating_sub(1);

    while left < right{
        if s[left] != s[right] {
            return false;
        };
        left += 1;
        right -= 1;
    };
    return true;
}


fn solve_fun(s: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    
    let mut left: usize = 0;
    let mut right: usize = s.len() - 1;

    while left < right {
        let l = s[left];
        let r = s[right];

        if !l.is_alphanumeric() {
            left += 1;
            continue;
        }
        if !r.is_alphanumeric() {
            right -= 1;
            continue;
        }

        if l.to_ascii_lowercase() != r.to_ascii_lowercase() {
            return false;
        };
        left += 1;
        right -= 1;
    }
    true
}
