use std::collections::HashSet;

fn get_box_id(row: usize, col: usize) -> usize {
    (row / 3) * 3 + (col / 3)
}

fn is_valid(
    box_set: &HashSet<char>,
    row_set: &HashSet<char>,
    col_set: &HashSet<char>,
    num: char,
) -> bool {
    !(box_set.contains(&num) || row_set.contains(&num) || col_set.contains(&num))
}

fn solve(
    board: &mut Vec<Vec<char>>,
    boxes: &mut Vec<HashSet<char>>,
    rows: &mut Vec<HashSet<char>>,
    cols: &mut Vec<HashSet<char>>,
    n: usize,
    r: usize,
    c: usize,
) -> bool {
    if r == n {
        return true;
    }

    let (next_r, next_c) = if c == n - 1 { (r + 1, 0) } else { (r, c + 1) };

    if board[r][c] == '.' {
        let box_id = get_box_id(r, c);

        for num in '1'..='9' {
            if is_valid(&boxes[box_id], &rows[r], &cols[c], num) {
                board[r][c] = num;
                boxes[box_id].insert(num);
                rows[r].insert(num);
                cols[c].insert(num);

                if solve(board, boxes, rows, cols, n, next_r, next_c) {
                    return true;
                }

                // backtrack
                board[r][c] = '.';
                boxes[box_id].remove(&num);
                rows[r].remove(&num);
                cols[c].remove(&num);
            }
        }
        false
    } else {
        solve(board, boxes, rows, cols, n, next_r, next_c)
    }
}

fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let n = board.len();
    let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); n];
    let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); n];
    let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); n];

    for i in 0..n {
        for j in 0..n {
            if board[i][j] != '.' {
                let box_id = get_box_id(i, j);
                boxes[box_id].insert(board[i][j]);
                rows[i].insert(board[i][j]);
                cols[j].insert(board[i][j]);
            }
        }
    }

    solve(board, &mut boxes, &mut rows, &mut cols, n, 0, 0);
}

fn main() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    solve_sudoku(&mut board);

    for row in board {
        println!("{:?}", row);
    }
}