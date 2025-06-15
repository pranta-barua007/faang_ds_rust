use std::collections::VecDeque;

fn rotten_oranges(matrix: &mut Vec<Vec<i32>>) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    if rows == 0 || cols == 0 {
        return 0;
    }

    const ROTTEN: i32 = 2;
    const FRESH: i32 = 1;

    let mut queue = VecDeque::new();
    let mut fresh_orange_count = 0;

    // Start Sequential traversal
    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == FRESH {
                fresh_orange_count += 1;
            }

            if matrix[row][col] == ROTTEN {
                queue.push_back((row, col));
            }
        }
    }

    let directions = vec![
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];

    let mut minutes = 0;
    let mut current_queue_size = queue.len();

    // Start BFS traversal
    while !queue.is_empty() {
        if current_queue_size == 0 {
            minutes += 1;
            current_queue_size = queue.len();
        }

        let (row, col) = queue.pop_front().unwrap();
        current_queue_size -= 1;

        for (dr, dc) in &directions {
            let next_row = row as isize + dr;
            let next_col = col as isize + dc;

            if next_row < 0
                || next_row >= rows as isize
                || next_col < 0
                || next_col >= cols as isize
            {
                continue;
            }

            let next_row = next_row as usize;
            let next_col = next_col as usize;

            if matrix[next_row][next_col] == FRESH {
                matrix[next_row][next_col] = ROTTEN;
                queue.push_back((next_row, next_col));
                fresh_orange_count -= 1;
            }
        }
    }

    if fresh_orange_count > 0 {
        return -1; // Not all oranges can be rotten
    }

    minutes
}

fn main() {
    let mut matrix = vec![
        vec![2, 1, 1, 0, 0],
        vec![1, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 1],
        vec![0, 1, 0, 0, 1],
    ];

    let result = rotten_oranges(&mut matrix);
    println!("minutes required for rotten oranges: {:?}", result);
}
