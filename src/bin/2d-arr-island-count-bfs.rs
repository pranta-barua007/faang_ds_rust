use std::{collections::VecDeque, vec};

fn traversal_sequential_bfs(matrix: &mut Vec<Vec<i32>>) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    if rows == 0 {
        return 0;
    }

    let directions = vec![
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];

    let mut island_count = 0;

    // Start Sequential traversal
    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == 1 {
                island_count += 1;
                matrix[row][col] = 0;

                // Start BFS
                let mut queue = VecDeque::new();
                queue.push_back(vec![row, col]);

                while !queue.is_empty() {
                    let current_position = queue.pop_front().unwrap();
                    let current_row = current_position[0];
                    let current_col = current_position[1];

                    // Check all 4 directions up, right, down, left
                    for (dr, dc) in &directions {
                        let next_row = current_row as isize + dr;
                        let next_col = current_col as isize + dc;

                        if next_row < 0
                            || next_row >= rows as isize
                            || next_col < 0
                            || next_col >= cols as isize
                        {
                            continue;
                        }

                        if matrix[next_row as usize][next_col as usize] == 1
                        {
                            matrix[next_row as usize][next_col as usize] = 0;
                            queue.push_back(vec![next_row as usize, next_col as usize]);
                        }
                    }
                }
            }

        }
    }

    island_count
}

fn main() {
    let mut matrix = vec![
        vec![1, 1, 1, 0, 0],
        vec![1, 1, 1, 0, 1],
        vec![0, 1, 0, 0, 1],
        vec![0, 0, 0, 1, 1],
    ];

    let result = traversal_sequential_bfs(&mut matrix);
    println!("island count: {:?}", result);
}
