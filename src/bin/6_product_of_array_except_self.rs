fn main() {
    let res = solve_fun(vec![1, 2, 3, 4]);
    println!("{:?}", res)
}

fn solve_fun(nums: Vec<i32>) -> Vec<i32> {
    let mut left = vec![];
    let mut prev = 1;

    for num in nums.iter() {
        left.push(prev);
        prev = prev * num;
    }

    let mut res: Vec<i32> = vec![1; nums.len()];
    prev = 1;
    for i in (0..nums.len()).rev(){
        res[i] = prev * left[i];
        prev = prev * nums[i];
    };
    res

}
