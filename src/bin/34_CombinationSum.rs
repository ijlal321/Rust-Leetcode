fn main() {
    let res = solve_fun(vec![2,3,5], 8);
    println!("{:?}", res)
}

fn solve_fun(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    back_track(&candidates, target, 0, &mut vec![], 0, &mut res);

    res
}

fn back_track(
    candidates: &Vec<i32>,
    target: i32,
    cur_index: i32,
    cur_vec: &mut Vec<i32>,
    cur_sum: i32,
    res: &mut Vec<Vec<i32>>,
) {
    if cur_sum == target {
        res.push(cur_vec.clone());
        return;
    }
    if cur_sum > target {
        return;
    }

    for i in (cur_index as usize)..candidates.len() {
        cur_vec.push(candidates[i]);
        back_track( candidates, target, i as i32, cur_vec, cur_sum + candidates[i], res);
        cur_vec.pop();
    }
}
