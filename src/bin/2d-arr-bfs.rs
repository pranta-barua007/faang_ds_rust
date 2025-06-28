use std::{collections::VecDeque, vec};

fn traversal_bfs(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut seen = vec![vec![false; cols]; rows];
    let mut values = vec![];

    let mut queue = VecDeque::new();
    queue.push_back(vec![0, 0]);

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

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let row = current[0] as isize;
        let col = current[1] as isize;

        if row < 0
            || row >= rows as isize
            || col < 0
            || col >= cols as isize
            || seen[row as usize][col as usize]
        {
            continue;
        }

        seen[row as usize][col as usize] = true;
        values.push(matrix[row as usize][col as usize]);

        for (dr, dc) in &directions {
            let new_row = (row + dr) as usize;
            let new_col = (col + dc) as usize;

            if new_row >= rows as usize || new_col >= cols as usize {
                continue;
            }

            queue.push_back(vec![new_row, new_col]);
        }
    }

    values
}

fn main() {
    let matrix = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
    ];

    let result = traversal_bfs(&matrix);
    println!("{:?}", result);
}
