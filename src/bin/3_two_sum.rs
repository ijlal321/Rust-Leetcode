fn main() {
    let res = solve_fun(vec![3,2,4],  6);
    println!("{:?}", res)
}

fn solve_fun(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut check_map = std::collections::HashMap::<i32, i32>::new();

    for (index, num) in nums.into_iter().enumerate(){

        if check_map.contains_key(&num){
            return vec![index as i32, check_map.get(&num).unwrap().clone()];
        }

        check_map.insert(target - num, index as i32);
    }

    vec![1,2] // just a dummy value, in case we cant find answer 

}
