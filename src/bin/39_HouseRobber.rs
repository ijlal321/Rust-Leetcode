fn main() {
    let res = solve_fun(vec![1, 2, 3, 1]);
    println!("{:?}", res)
}

fn solve_fun(nums: Vec<i32>) -> i32 {
    let mut visited: std::collections::HashMap<usize, i32> = std::collections::HashMap::new();
    rob_house(&nums, 0, &mut visited)
}

fn rob_house(nums: &Vec<i32>, cur_index: usize, visited: &mut std::collections::HashMap<usize, i32>) -> i32 {
    if cur_index >= nums.len() {
        return 0;
    }
    if let Some(sum) = visited.get(&cur_index){
        return *sum;
    }

    let sum = std::cmp::max(
        rob_house(nums, cur_index + 1, visited),
        rob_house(nums, cur_index + 2, visited) + nums[cur_index],
    );
    visited.insert(cur_index, sum);
    sum
}
