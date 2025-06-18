
fn main() {
    let res = solve_fun(vec![1,1,1,2,2,3],2);
    println!("{:?}", res)
}

fn solve_fun(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count_map = std::collections::HashMap::<i32, i32>::new();

    let mut fre_vec: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];

    for num in nums{
        *count_map.entry(num).or_insert(0) += 1;
    };

    // println!("{:?}", count_map);

    for (val, cnt) in count_map{
        fre_vec[cnt as usize].push(val);
    }

    // println!("{:#?}", fre_vec);

    let mut res = vec![];
    for freq in fre_vec.into_iter().rev(){
        for num in freq{
            res.push(num);
            if res.len() as i32 == k{
                return res;
            }
        }
    }
    return vec![];
}
