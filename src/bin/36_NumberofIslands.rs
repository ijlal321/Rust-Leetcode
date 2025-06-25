fn main() {
    let grid: Vec<Vec<char>> = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    let res = solve_fun(grid);
    println!("{:?}", res)
}

fn solve_fun(grid: Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    let mut visited: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();

    for r in 0..grid.len(){
        for c in 0..grid[0].len(){
            if !visited.contains(&(r,c)){
                res += find_land(&grid, r, c, &mut visited);
            }
        }
    };

    res
}

fn find_land(grid: &Vec<Vec<char>>, r: usize, c: usize, visited: &mut std::collections::HashSet<(usize, usize)>) -> i32{

    if r >= grid.len() || c >= grid[0].len() || visited.contains(&(r,c)) || grid[r][c] == '0'{
        return 0;
    } 

    visited.insert((r,c));

    find_land(grid, r + 1, c, visited);
    if r != 0 { find_land(grid, r - 1, c, visited);}
    find_land(grid, r, c + 1, visited);
    if c != 0 { find_land(grid, r, c - 1, visited);}


    1
}