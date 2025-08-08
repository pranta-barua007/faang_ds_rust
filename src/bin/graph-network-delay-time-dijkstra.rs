use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

type Source = i32;
type Target = i32;
type Weight = i32;
type Edge = (Source, Target, Weight);


// Space and time complexity:

// Time Complexity:
// - Building the adjacency list takes O(E), where E is the number of edges.
// - Dijkstra's algorithm involves priority queue operations: for each of the E edges, we perform
//   a push and pop operation, each taking O(log N) time. Therefore, the total time complexity for
//   these operations is O(E log N).
// - Thus, the overall time complexity is O(E log N), dominated by the priority queue operations.
// Final Time Complexity: O(E log N)

// Time Complexity (Detailed): 
// Initial Time Complexity: O(2N+E+(E.logN + N.logN)) 
// where: 
// N is the number of nodes and E is the number of edges.
// - O(2N) is for initializing the adjacency list and distances.
// - O(E) is for building the adjacency list.
// - O(E.logN) is for the priority queue operations in Dijkstra's algorithm.
// - O(N.logN) is for the priority queue operations when processing nodes.
// we are assure that E > N. and we can drop the N.logN term. also 2N and E are constants so those also dropped.
// Final Time Complexity: O(E.logN) because we are using Dijkstra's algorithm which has a time complexity of O(E.logN) when using a priority queue.

// Space Complexity: O(N + E)
// Where:
// N -> space for the priority queue, visited set, and distance array, each requiring O(N) space.
// E -> space for the adjacency list, which requires O(N + E) space due to storing both nodes and edges.
// Combining the above, we get O(N + N + E) = O(2N + E), which simplifies to O(N + E) by dropping the constant 2.
// Final Space Complexity: O(N + E)
fn network_delay_time(times: Vec<Edge>, n: i32, k: i32) -> i32 {
    let mut adjacecncy_list: Vec<Vec<(Target, Weight)>> = vec![Vec::new(); n as usize];
    let mut distances: Vec<i32> = vec![i32::MAX; n as usize];

    // Build the adjacency list (make sure to adjust to 0-based index)
    for (source, target, weight) in times {
        adjacecncy_list[(source - 1) as usize].push((target - 1, weight));
    }

    // print the adjacency list for debugging
    println!("Adjacency List: {:?}", adjacecncy_list);

    // Initialize distances for the starting node
    distances[(k - 1) as usize] = 0;

    // Priority queue(min-heap) for Dijkstra's algorithm
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, (k - 1) as usize))); // (distance, node)

    // Set to keep track of visited nodes
    let mut visited = HashSet::new();

    while let Some(Reverse((current_distance, current_vertex))) = heap.pop() {
        // Skip the node if it has already been processed
        if visited.contains(&current_vertex) {
            continue;
        }

        visited.insert(current_vertex);

        // Update distances for adjacent nodes
        for &(adjacent_vertex, weight) in &adjacecncy_list[current_vertex] {
            let new_distance = current_distance + weight;
            if new_distance < distances[adjacent_vertex as usize] {
                distances[adjacent_vertex as usize] = new_distance;
                heap.push(Reverse((new_distance, adjacent_vertex as usize)));
            }
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
    println!("{}", network_delay_time(times, nodes, starting_node));
}
