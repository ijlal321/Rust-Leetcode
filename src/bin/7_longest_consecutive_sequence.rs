use std::cmp::max;

fn main() {
    let res = solve_fun(vec![100,4,200,1,3,2]);
    println!("{:?}", res)
}

fn solve_fun(nums: Vec<i32>) -> i32{

    let nums_set = nums.iter().cloned().collect::<std::collections::HashSet<i32>>();

    let mut longest = 0;

    for num in nums_set.iter(){
        if nums_set.contains(&(num - 1)){
            continue;
        }
        let mut length = 1;
        while nums_set.contains(&(num + length)) {
            length += 1;
        }
        longest = max(longest, length);

    }

    longest
}
