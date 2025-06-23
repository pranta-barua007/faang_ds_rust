fn dfs(
    grid: &mut Vec<Vec<i32>>,
    row: isize,
    col: isize,
    directions: &[(isize, isize)],
    rows: isize,
    cols: isize,
    count: i32,
) {
    if row < 0 || row >= rows || col < 0 || col >= cols 
       || count > grid[row as usize][col as usize] {
        return;
    }

    grid[row as usize][col as usize] = count;

    for (dr, dc) in directions.iter() {
        let next_row = row + dr;
        let next_col = col + dc;
        dfs(grid, next_row, next_col, directions, rows, cols, count + 1);
    }
}

fn walls_and_gates(grid: &mut Vec<Vec<i32>>, gate: i32) {
    let rows = grid.len();
    let cols = grid[0].len();
    if rows == 0 || cols == 0 {
        return;
    }

    let directions = [
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];

    // Start Sequential traversal
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == gate {
                // Start DFS
                dfs(
                    grid,
                    row as isize,
                    col as isize,
                    &directions,
                    rows as isize,
                    cols as isize,
                    0
                );
            }
        }
    }
}

fn main() {
    const INF: i32 = 2147483647;
    const WALL: i32 = -1;
    const GATE: i32 = 0;
    // Example matrix representing walls and gates
    let mut test_matrix = vec![
        vec![INF, WALL, GATE, INF],
        vec![INF, INF, INF, GATE],
        vec![INF, WALL, INF, WALL],
        vec![GATE, WALL, INF, INF]
    ];

    walls_and_gates(&mut test_matrix, GATE);
   
    print!("{:?}\n", test_matrix);
}
