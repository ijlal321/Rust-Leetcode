fn main() {
    let res = solve_fun(vec![1]);
    println!("{:?}", res)
}

fn solve_fun(nums: Vec<i32>) -> i32{
    let mut left = 0; 
    let mut right = nums.len().saturating_sub(1);

    while left <= right{
        let mid  = (left + right) / 2;
        if nums[left] <= nums[right]{
            return nums[left];
        };

        if nums[mid] >= nums[left] {
            left = mid + 1;
        }
        else{
            right = mid;
        }

    }   


    -1 

}
