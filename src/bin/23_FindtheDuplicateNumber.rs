fn main() {
    let res = solve_fun(vec![1, 3, 4, 2, 2]);
    println!("{:?}", res)
}

fn solve_fun(nums: Vec<i32>) -> i32 {
    let mut nums = nums;

    for i in 0..nums.len() {
        let num = nums[i].abs();
        let idx = num as usize - 1;

        if nums[idx] < 0 {
            return num;
        }

        nums[idx] *= -1;
    }
    0
}
