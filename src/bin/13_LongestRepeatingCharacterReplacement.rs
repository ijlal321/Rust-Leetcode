fn main() {
    let res = solve_fun_non_maxf( String::from("AABABBA"), 1);
    println!("{:?}", res)
}

    // a better option in my opinion
fn solve_fun_non_maxf(s: String, k: i32) -> i32{
    let mut res: i32 = 0;

    let mut l = 0;
    let mut count_map: std::collections::HashMap<char, i32> = std::collections::HashMap::new();

    let chars: Vec<char> = s.chars().collect();

    for r in 0..chars.len(){

        *count_map.entry(chars[r]).or_insert(0) += 1;

        while r as i32 - l as i32 + 1 - count_map.values().max().unwrap() > k {
            *count_map.entry(chars[l]).or_insert(0) -= 1;
            l += 1;
        };

        res = std::cmp::max(res, r as i32 - l as i32 + 1 );

    }

    res
}


fn solve_fun(s: String, k: i32) -> i32{
    let mut res: i32 = 0;

    let mut l = 0;
    let mut count_map: std::collections::HashMap<char, i32> = std::collections::HashMap::new();

    let chars: Vec<char> = s.chars().collect();

    let mut maxf = 0;

    for r in 0..chars.len(){

        *count_map.entry(chars[r]).or_insert(0) += 1;
        maxf = std::cmp::max(maxf, *count_map.get(&chars[r]).unwrap());

        while r as i32 - l as i32 + 1 - maxf > k {
            *count_map.entry(chars[l]).or_insert(0) -= 1;
            l += 1;
        };

        res = std::cmp::max(res, r as i32 - l as i32 + 1 );

    }

    res
}
