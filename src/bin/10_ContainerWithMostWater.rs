use std::cmp::{max, min};

fn main() {
    let res = solve_fun(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("{:?}", res)
}

fn solve_fun(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len().saturating_sub(1);

    let mut res = 0;

    while left < right {
        res = max(
            res,
            min(height[left], height[right]) * (right - left) as i32,
        );

        if height[left] <= height[right] {
            left += 1;
        }else {
            right -= 1;
        };
    }

    res
}
