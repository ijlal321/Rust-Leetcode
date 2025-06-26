fn main() {
    let res = solve_fun(45);
    println!("{:?}", res);
}

fn solve_fun(n: i32) -> i32 {
    let mut visited: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    find_ways(n, &mut visited)
}

fn find_ways(n: i32, visited: &mut std::collections::HashMap<i32, i32>) -> i32 {
    if n <= 2 {
        return n;
    }
    if visited.contains_key(&n) {
        return *visited.get(&n).unwrap();
    }
    let sum = find_ways(n - 1, visited) + find_ways(n - 2, visited);
    visited.insert(n, sum);
    sum
}
