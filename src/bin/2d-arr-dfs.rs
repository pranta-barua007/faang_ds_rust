fn dfs(
    matrix: &Vec<Vec<i32>>,
    row: usize,
    col: usize,
    seen: &mut Vec<Vec<bool>>,
    values: &mut Vec<i32>,
) {
    if seen[row][col] {
        return;
    }

    seen[row][col] = true;
    values.push(matrix[row][col]);

    // 8 directions: up, right, down, left, and diagonals
    let directions = vec![
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
                 // (-1, -1), // top-left
                 // (-1, 1),  // top-right
                 // (1, -1),  // bottom-left
                 // (1, 1),   // bottom-right
    ];

    for (dr, dc) in directions {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if new_row >= 0
            && new_row < matrix.len() as isize
            && new_col >= 0
            && new_col < matrix[0].len() as isize
        {
            dfs(matrix, new_row as usize, new_col as usize, seen, values);
        }
    }
}

fn traversal_dfs(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut seen = vec![vec![false; cols]; rows];
    let mut values = vec![];

    dfs(matrix, 0, 0, &mut seen, &mut values);

    values
}

fn main() {
    let matrix = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
    ];

    let result = traversal_dfs(&matrix);
    println!("{:?}", result);
}
