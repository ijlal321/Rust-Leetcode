fn main() {
    let _res = solve_fun(vec![1, 2, 3, 4]);
}

fn solve_fun(nums: Vec<i32>) -> bool {
    let mut nums_hash_set = std::collections::HashSet::<i32>::new();

    for num in nums {
        if nums_hash_set.contains(&num){
            return true;
        };
        nums_hash_set.insert(num);
    };
    return false;
}
