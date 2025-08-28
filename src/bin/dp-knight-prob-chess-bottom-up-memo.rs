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

// Bottom up Approach
// identify the recusrive depth driving factor, which is k here
// here we will start from 0 will go to k
// at k = 0 we know the prob of knight is 1 at (row, column) and 0 at all other places
// then we will use our dp at k=0 to find the prob at k=1 then so on till k

// Time and Space Complexity

// Time Complexity: O(8 * 2.n^2 * k) -> drop the constant 8 and 2 -> O(n^2 * k)
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
    let mut dp = vec![vec![vec![0.0; n as usize]; n as usize]; (k + 1) as usize];
    dp[0][row as usize][column as usize] = 1.0;

    for step in 1..=k as usize {
        for r in 0..n as usize {
            for c in 0..n as usize {
                let mut prob = 0.0;
                for (dr, dc) in KNIGHT_DIRECTIONS.iter() {
                    let dr = *dr as i32;
                    let dc = *dc as i32;
                    let new_r = r as i32 + dr;
                    let new_c = c as i32 + dc;
                    if new_r >= 0 && new_r < n && new_c >= 0 && new_c < n {
                        prob += dp[step - 1][new_r as usize][new_c as usize];
                    }
                }
                dp[step][r][c] = prob / 8.0;
            }
        }
    }

    let mut total_prob = 0.0;
    for r in 0..n as usize {
        for c in 0..n as usize {
            total_prob += dp[k as usize][r][c]; 
        }
    }

    total_prob

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
