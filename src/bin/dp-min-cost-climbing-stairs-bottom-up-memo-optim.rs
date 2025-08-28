// BOTTOM UP APPROACH -> always start form first element i.e (0) and go forward

// STEP 1: Find Recurrence Relationship
// STEP 2: Find Base Cases, and Recursive Case
// STEP 3: Find known and unknown subproblems
// STEP 4: Find and Memoize/Cache Repeated Subproblems/Calculations
// STEP 5: Find unused memory or cachce and try to optimize

// Recurrence Relationship
// minCost[i] = cost[i] + min(minCost[i-1], minCost[i-2]) 
//  minCost[0] = cost[0]
//  minCost[1] = cost[1]

// Base Cases
// minCost[0] = cost[0]
// minCost[1] = cost[1]

// Recursive Case
// minCost[i] = cost[i] + min(minCost[i-1], minCost[i-2])

// State Space Tree (TOP DOWN)
//                                      mC(i)
//                                      /  \
//                                     /    \
//                     c[i] + min(mC(i-1) , mC(i-2))
//                                / \          / \
//                               /   \        /   \
//        c[i-1] + min(min(mC(i-2), mC(i-3)) , c[i-2] + min(mC(i-3), mC(i-4)))
//
//                            untill i == 0 || i == 1


// Find known and unknown subproblems
// For our particular problem
// known -> i, i-1 as we can take 2 steps at a time
// unknown -> i-2, i-3, i-4, i-5, i-6, i-7, i-8, i-9 .... so on


// State Spce Tree (BOTTOM UP)
//
//                      mC(4)
//                      / \
//                     /   \
//                    /     \
//                   /       \
//           c[4] + min(mC(3) , )
//                 /     \     \
//                /       \     \
//               /         \     |
//              /           \    |
//             /             \   |
// c[3] + min(       ,       mC(2))
//           /             /       \
//          /             /         \
//         /             /           \
//        /             /             \        
//      c[2] + min(mC(1)        ,    mC(0))
//
//    START FROM BOTTOM with 1st element (known step) and go up         


// Finding Repetitive Step/Subproblem/Calculation from state space tree
// i-1, i-2 and i-3 are the repetitive steps and so on


// Memoization
// we can cache the repetitive calculation in same data structure of the problem
// here our cost data structure is an array so our cache DS will also be an array
// we will cache or memoize caculation for each step in the cache array
// so at any step i if we have dp[i] we use it else we dp[i] = calculation


// Find unused memory and Optimize Memoization
// here our dp array is of size N
// but we just rely on the last 2 elements
// we can optimize our memoization by using 2 variables instead of array


// Time and Space Complexity

// Time Complexity: O(N) with memoization
// here we iterate over the whole N items in arr 

// Space Complexity: O(N) reduced to -> O(1)
// here we use 2 variables instead of array so no scaling DS

fn min_cost_climbing_stairs(cost: &Vec<i32>) -> i32 {
    let n = cost.len();

    if n == 0 {
        return 0;
    }

    if n == 1 {
        return cost[0];
    }

    let mut dp_prev = cost[0];
    let mut dp_current = cost[1];
    
    for i in 2..n {
        let current = cost[i] + dp_prev.min(dp_current);
        dp_prev = dp_current;
        dp_current = current;
    }

    dp_prev.min(dp_current)
}


fn main() {
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    println!("Min cost: {}", min_cost_climbing_stairs(&cost));
}