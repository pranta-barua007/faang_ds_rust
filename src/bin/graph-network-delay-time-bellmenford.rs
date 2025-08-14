type Source = i32;
type Target = i32;
type Weight = i32;
type Edge = (Source, Target, Weight);

// Space and time complexity:
// O(N)
// Time Complexity:
// O(N + N.E) where N.E is for inner loop and N is for outer loop
// N can be dropped as it is a constant factor
// Final Time Complexity: O(N.E)

fn network_delay_time_bellmenford(times: Vec<Edge>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;

    let mut distances: Vec<i32> = vec![i32::MAX; n];
    distances[k - 1] = 0; // Starting node distance is 0

    for _ in 0..n - 1 {
        let mut updated = false;
        for &(source, target, weight) in &times {
            let source_index = (source - 1) as usize;
            let target_index = (target - 1) as usize;

            if distances[source_index] != i32::MAX
                && distances[source_index] + weight < distances[target_index]
            {
                distances[target_index] = distances[source_index] + weight;
                updated = true;
            }
        }

        if !updated {
            break; // No updates means we can stop early
        }
    }

    // Get the maximum distance in the distances array
    let max_distance = distances.into_iter().max().unwrap_or(i32::MAX);

    // If we have a node that is still at "infinity", return -1
    if max_distance == i32::MAX {
        -1
    } else {
        max_distance
    }
}

fn main() {
    let times: Vec<Edge> = vec![
        (1, 2, 9),
        (1, 4, 2),
        (2, 5, 1),
        (4, 2, 4),
        (4, 5, 6),
        (3, 2, 3),
        (5, 3, 7),
        (3, 1, 5),
    ];
    let nodes = 5;
    let starting_node = 1;
    println!(
        "{}",
        network_delay_time_bellmenford(times, nodes, starting_node)
    );
}
