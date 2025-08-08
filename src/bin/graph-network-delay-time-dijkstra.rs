use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

type Source = i32;
type Target = i32;
type Weight = i32;
type Edge = (Source, Target, Weight);

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
