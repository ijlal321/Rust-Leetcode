
fn main() {
    let res = solve_fun(String::from("anagram"), String::from("nagaram"));
    println!("{:?}", res);
}

fn solve_fun(s: String, t: String) -> bool{
    if s.len() != t.len() {
        return false;
    };

    let mut comp_vec = vec![0; 26];

    for (s_char, t_char) in s.chars().zip(t.chars()){
        comp_vec[s_char as usize - 'a' as usize] += 1;
        comp_vec[t_char as usize-  'a' as usize] -= 1;
        
    };

    return  comp_vec.iter().all(|quantity| *quantity == 0);

}
