
fn main() {
    let res = solve_fun(vec![7,6,4,3,1]);
    println!("{:?}", res)
}

fn solve_fun(prices: Vec<i32>) -> i32{
    let mut res = 0;

    let mut prev_smallest = prices[0];

    for price in prices{
        res = std::cmp::max(res, price - prev_smallest);

        if prev_smallest > price{
            prev_smallest = price;
        }
    }

    res
}
