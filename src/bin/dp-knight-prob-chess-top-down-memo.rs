// TOP DOWN APPROACH -> always start form last i.e (n) and go backwards to the 0

// 🐴 KNIGHT_DIRECTIONS = [
//   [-2, -1],
//   [-2, 1],
//   [-1, 2],
//   [1, 2],
//   [2, 1],
//   [2, -1],
//   [1, -2],
//   [-1, -2]
// ];


// STEP 1: Find Recurrence Relationship
// STEP 2: Find Base Cases, and Recursive Case

// Recurrence Relationship
// knightProb(k, r, c) = (1/8) * sum(knightProb(k-1, r', c')) for all valid (r', c')
// where r' = r + x and c' = c + y for each (x, y) in KNIGHT_DIRECTIONS

// Base Cases
// c < 0 || c >= n || r < 0 || r >= n -> return 0
// knightProb(0, r, c) = 1 if (r, c) is on the board else 0 (when off the board)

// Recursive Case
// knightProb(k, r, c) = (1/8) * sum(knightProb(k-1, r', c')) for all valid (r', c')

// Time and Space Complexity

// Time Complexity: O(n^2 * k)
// here n^2 * k is the number of possible states, because we have n*n positions
// and k moves, so total n*n*k states and for every state we are doing

// Space Complexity: O(n^2 * k)
// where n^2 * k is the size of the dp array we are using to store the results of subproblems

const KNIGHT_DIRECTIONS: [(isize, isize); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, 2),
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
];

fn knight_prob(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let mut dp = vec![vec![vec![-1.0; n as usize]; n as usize]; (k + 1) as usize];
    knight_recursive(n, k, row, column, &mut dp)
}

fn knight_recursive(n: i32, k: i32, row: i32, column: i32, dp: &mut Vec<Vec<Vec<f64>>>) -> f64 {
    if row < 0 || row >= n || column < 0 || column >= n {
        return 0.0;
    }

    if k == 0 {
        return 1.0;
    }

    if dp[k as usize][row as usize][column as usize] != -1.0 {
        return dp[k as usize][row as usize][column as usize];
    }

    let mut prob = 0.0;
    for (dr, dc) in KNIGHT_DIRECTIONS.iter() {
        let dr = *dr as i32;
        let dc = *dc as i32;
        prob += knight_recursive(n, k - 1, row + dr, column + dc, dp);
    }

    dp[k as usize][row as usize][column as usize] = prob / 8.0;

    dp[k as usize][row as usize][column as usize]
}


fn main() {
    let n = 3;
    let k = 2;
    let row = 0;
    let column = 0;
    println!(
        "Knight Probability: {}",
        knight_prob(n, k, row, column)
    );
}
