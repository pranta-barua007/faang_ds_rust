fn dfs(grid: &mut Vec<Vec<i32>>, row: isize, col: isize, directions: &[(isize, isize)]) {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    if row < 0 || row >= rows || col < 0 || col >= cols {
        return;
    }

    if grid[row as usize][col as usize] == 1 {
        // Mark as visited
        grid[row as usize][col as usize] = 0;

        for (dr, dc) in directions.iter() {
            let next_row = row + dr;
            let next_col = col + dc;
            dfs(grid, next_row, next_col, directions);
        }
    }
}

fn number_of_islands(mut grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let directions = [
        (-1, 0),  // up
        (0, 1),   // right
        (1, 0),   // down
        (0, -1)   // left
    ];

    let mut island_count = 0;

    // Start Sequential traversal
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 1 {
                island_count += 1;
                // Start DFS
                dfs(&mut grid, row as isize, col as isize, &directions);
            }
        }
    }

    island_count
}

fn main() {
    let test_matrix = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1],
        vec![0, 1, 1, 1, 0],
        vec![1, 0, 1, 0, 1],
    ];

    let result = number_of_islands(test_matrix);
    println!("Number of islands: {}", result);
}
