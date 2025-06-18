fn main() {
    let res = solve_fun(vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat")
        ]);
    println!("{:?}", res)
}

fn solve_fun(strs: Vec<String>) -> Vec<Vec<String>>{


    let mut matching_map = std::collections::HashMap::<Vec<i32>, Vec<String>>::new();

    for word in strs{
        let res_vec = create_vector(&word);
        matching_map.entry(res_vec).or_insert(vec![]).push(word);
    }
    let res = matching_map.values().cloned().collect();
    return res; 
}


fn create_vector(word: &String)-> Vec<i32> {
    let mut res_vec = vec![0; 26];
    
    for letter in word.chars(){
        res_vec[letter as usize - 'a' as usize] += 1;
    };

    res_vec
}
