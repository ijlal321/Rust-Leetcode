fn main() {
    let res = solve_fun(String::from("abcabcbb"));
    println!("{:?}", res)
}

fn solve_fun(s: String) -> i32 {
    let mut max_len = 0;
    let mut left = 0;
    let mut set = std::collections::HashSet::new();
    let chars: Vec<char> = s.chars().collect();

    for right in 0..chars.len() {
        while set.contains(&chars[right]) {
            set.remove(&chars[left]);
            left += 1;
        }
        set.insert(chars[right]);
        max_len = max_len.max(right - left + 1);
    }

    max_len as i32
}
