fn main() {
    let res = solve_fun(vec![4, 5, 6, 7, 8, 1, 2, 3], 2);
    //      0       4     7
    println!("{:?}", res)
}

fn solve_fun(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if target == nums[mid] {
            return mid as i32;
        }

        if nums[left] <= nums[mid] {
            // meaning if left side sorted
            if nums[left] <= target && target <= nums[mid] {
                right = mid;
            } else {
                left = mid + 1;
            };
        } else {
            // meaning if right side sorted
            if target >= nums[mid] && target <= nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
    };
    -1
}
