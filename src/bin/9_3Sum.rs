
fn main() {
    let res = solve_fun(vec![-1,0,1,2,-1,-4]);
    println!("{:?}", res)
}

fn solve_fun(nums: Vec<i32>) -> Vec<Vec<i32>>{
    
    let mut sorted_nums = nums;
    sorted_nums.sort();

    let mut res: Vec<Vec<i32>> = vec![];

    for (i, a) in sorted_nums.iter().enumerate(){
        if i > 0 && sorted_nums[i-1] == *a{
            continue;
        }
        let mut left = i + 1;
        let mut right = sorted_nums.len() - 1;

        while left < right {
            let three_sum = a + sorted_nums[left] + sorted_nums[right];

            if three_sum > 0{
                right -= 1;
            }
            else if three_sum < 0 {
                left += 1;
            }
            else {
                res.push(vec![*a, sorted_nums[left], sorted_nums[right]]);
                left += 1;
                while sorted_nums[left] == sorted_nums[left - 1] && left < right{
                    left += 1;
                };
            };
        }
    }

    res
}
