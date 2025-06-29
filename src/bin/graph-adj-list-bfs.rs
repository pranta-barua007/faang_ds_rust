use std::{collections::VecDeque};

fn traversal_bfs(adjacency_list: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited = vec![false; adjacency_list.len()];
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    // Start BFS from the first node (0)
    queue.push_back(0);
    visited[0] = true;

    while let Some(node) = queue.pop_front() {
        result.push(node);

        for &neighbor in &adjacency_list[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    result
}

fn main() {
    // Example adjacency matrix
    // This is a 2D array representation of a graph
    // where each inner vector represents the connections of a node.
    // For example, node 0 is connected to nodes 1 and 3.
    let adjacency_list = vec![
        vec![1, 3],
        vec![0],
        vec![3, 8],
        vec![0, 2, 4, 5],
        vec![3, 6],
        vec![3],
        vec![4, 7],
        vec![6],
        vec![2]
    ];

    let result = traversal_bfs(&adjacency_list);
    println!("BFS Traversal Result: {:?}", result);
}
