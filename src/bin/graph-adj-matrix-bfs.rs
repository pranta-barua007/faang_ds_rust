use std::{collections::VecDeque};

fn traversal_bfs(adjacency_matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    let n = adjacency_matrix.len();
    let mut seen = vec![false; n];
    let mut values = vec![];
    let mut queue = VecDeque::new();

    // Start BFS from the first node (index 0)
    queue.push_back(0);
    seen[0] = true;

    while let Some(node) = queue.pop_front() {
        values.push(node as i32); // Store the node value
        
        let connections = &adjacency_matrix[node];
        for (neighbor, &is_connected) in connections.iter().enumerate() {
            if is_connected == 1 && !seen[neighbor] {
                seen[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }


    values
}

fn main() {
    // Example adjacency matrix
    // This is a 2D array representation of a graph
    // where 1 indicates an edge between nodes and 0 indicates no edge.
    // The graph is undirected, so the matrix is symmetric.
    let adjacency_matrix = vec![
        vec! [0, 1, 0, 1, 0, 0, 0, 0, 0],
        vec! [1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec! [0, 0, 0, 1, 0, 0, 0, 0, 1],
        vec! [1, 0, 1, 0, 1, 1, 0, 0, 0],
        vec! [0, 0, 0, 1, 0, 0, 1, 0, 0],
        vec! [0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec! [0, 0, 0, 0, 1, 0, 0, 1, 0],
        vec! [0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec! [0, 0, 1, 0, 0, 0, 0, 0, 0],
    ];

    let result = traversal_bfs(&adjacency_matrix);
    println!("BFS Traversal Result: {:?}", result);
}
