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

// Time Complexity: O(8^K)
// here 8^K is the number of possible paths, because ther recursion goes in tree manner
// for every k we have 8 possible combinations of knight moves

// Space Complexity: O(8^K)
// wehere 8^K is the number of function calls for recursion call stack

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
    if row < 0 || row >= n || column < 0 || column >= n {
        return 0.0;
    }

    if k == 0 {
        return 1.0;
    }

    let mut prob = 0.0;
    for (dr, dc) in KNIGHT_DIRECTIONS.iter() {
        let dr = *dr as i32;
        let dc = *dc as i32;
        prob += knight_prob(n, k - 1, row + dr, column + dc);
    }

    prob / 8.0
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
