// TOP DOWN APPROACH -> always start form last element i.e (n)

// STEP 1: Find Recurrence Relationship
// STEP 2: Find Base Cases, and Recursive Case
// STEP 3: Find and Memoize/Cache Repeated Subproblems/Calculations

// Recurrence Relationship
// minCost[i] = cost[i] + min(minCost[i-1], minCost[i-2]) 
//  minCost[0] = cost[0]
//  minCost[1] = cost[1]

// Base Cases
// minCost[0] = cost[0]
// minCost[1] = cost[1]

// Recursive Case
// minCost[i] = cost[i] + min(minCost[i-1], minCost[i-2])

// State Space Tree
//                                      mC(i)
//                                      /  \
//                                     /    \
//                     c[i] + min(mC(i-1) , mC(i-2))
//                                / \          / \
//                               /   \        /   \
//        c[i-1] + min(min(mC(i-2), mC(i-3)) , c[i-2] + min(mC(i-3), mC(i-4)))
//
//                            untill i == 0 || i == 1

// Finding Repetitive Step/Subproblem/Calculation from state space tree
// i-1, i-2 and i-3 are the repetitive steps and so on

// Memoization
// we can cache the repetitive calculation in same data structure of the problem
// here our cost data structure is an array so our cache DS will also be an array
// we will cache or memoize caculation for each step in the cache array
// so at any step i if we have dp[i] we use it else we dp[i] = calculation


// Time and Space Complexity

// Time Complexity: O(2^N) reduced to -> O(N) with memoization
// here for step N we still do 2 recursive calls at every branch of the tree
// but the optimization is that we cache the calculation for every step
// so for any step i if we already calculated and saved it dp[i]
// so on every brach when we encounter dp[i] we use it, eventually cutting down that branch call
// so our call is reduced to N

// Space Complexity: O(N)
// wehere N is number of function calls for recursion call stack

fn min_cost_climbing_stairs(cost: &Vec<i32>) -> i32 {
    let n = cost.len() as isize;
    let mut dp = vec![0; n as usize];
    min_cost(n - 1, cost, &mut dp).min(min_cost(n - 2, cost, &mut dp))
}

fn min_cost(i: isize, cost: &Vec<i32>, dp: &mut Vec<i32>) -> i32 {
    if i < 0 {
        return 0;
    }
    if i == 0 || i == 1 {
        return cost[i as usize];
    }

    if dp[i as usize] != 0 {
        return dp[i as usize];
    }

    dp[i as usize] = cost[i as usize] + min_cost(i - 1, cost, dp).min(min_cost(i - 2, cost, dp));

    dp[i as usize]
}


fn main() {
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    println!("Min cost: {}", min_cost_climbing_stairs(&cost));
}