// TOP DOWN APPROACH -> always start form last element i.e (n) and go backwards

// STEP 1: Find Recurrence Relationship
// STEP 2: Find Base Cases, and Recursive Case

// Recurrence Relationship
// minCost[i] = cost[i] + min(minCost[i-1], minCost[i-2])
//  minCost[0] = cost[0]
//  minCost[1] = cost[1]

// Base Cases
// minCost[0] = cost[0]
// minCost[1] = cost[1]

// Recursive Case
// minCost[i] = cost[i] + min(minCost[i-1], minCost[i-2])

// Time and Space Complexity

// Time Complexity: O(2^N)
// here 2^N is the number of possible paths, because ther recursion goes in tree manner
// where for every root i there are i-1 and i-2 recursive calls at every branch of the tree
// as we have N path so at every call we have 2^N recursive calls

// Space Complexity: O(N)
// wehere N is number of function calls for recursion call stack

fn min_cost_climbing_stairs(cost: &Vec<i32>) -> i32 {
    let n = cost.len() as isize;

    if n == 1 {
        return cost[0];
    }

    min_cost(n - 1, cost).min(min_cost(n - 2, cost))
}

fn min_cost(i: isize, cost: &Vec<i32>) -> i32 {
    if i < 0 {
        return 0;
    }

    if i == 0 || i == 1 {
        return cost[i as usize];
    }

    cost[i as usize] + min_cost(i - 1, cost).min(min_cost(i - 2, cost))
}

fn main() {
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    println!("Min cost: {}", min_cost_climbing_stairs(&cost));
}
