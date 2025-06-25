fn main() {
    let heights: Vec<Vec<i32>> = [
        [1,2,2,3,5],
        [3,2,3,4,4],
        [2,4,5,3,1],
        [6,7,1,4,5],
        [5,1,1,2,4]
    ].iter().map(|row| row.to_vec()).collect();
    let res = solve_fun();
    println!("{:?}", res)
}

fn solve_fun(){

}
