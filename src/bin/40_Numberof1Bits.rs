fn main() {
    let res = solve_fun(11);
    println!("{:?}", res)
}

fn solve_fun(n: i32) -> i32{
    let mut res = 0;
    let mut n = n;

    while n > 0 {
        res += n % 2;
        n = n >> 1;
    }

    res
}
