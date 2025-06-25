fn main() {
    let res = solve_fun(
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        String::from("ABCCED"),
    );
    println!("{:?}", res)
}

fn solve_fun(board: Vec<Vec<char>>, word: String) -> bool {
    let mut visited = std::collections::HashSet::<(usize, usize)>::new();
    let words_vec: Vec<char> = word.chars().collect();

    for r in 0..board.len() {
        for c in 0..board[0].len() {
            if back_track(&board, r, c, 0, &mut visited, & words_vec) {
                return true;
            }
        }
    }
    false
}

fn back_track(
    board: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    cur_idx: usize,
    visited: &mut std::collections::HashSet<(usize, usize)>,
    word: & Vec<char>,
) -> bool {

    if cur_idx == word.len() {
        return true;
    }

    if r < 0
        || c < 0
        || r >= board.len()
        || c >= board[0].len()
        || visited.contains(&(r, c))
        || word[cur_idx] != board[r][c]
    {
        return false;
    };

    visited.insert((r,c));

    let res = back_track(board,     r+1, c, cur_idx+1, visited, word) ||
        (r != 0 && back_track(board,     r-1, c, cur_idx+1, visited, word) )||
        back_track(board,     r, c+1, cur_idx+1, visited, word) ||
        (c != 0 && back_track(board,     r, c-1, cur_idx+1, visited, word));

    visited.remove(&(r,c));

    res
}
