fn dfs(adjacency_matrix: &Vec<Vec<i32>>, node: usize, seen: &mut Vec<bool>, values: &mut Vec<i32>) {
    seen[node] = true;
    values.push(node as i32); // Store the node value

    for (neighbor, &is_connected) in adjacency_matrix[node].iter().enumerate() {
        if is_connected == 1 && !seen[neighbor] {
            dfs(adjacency_matrix, neighbor, seen, values);
        }
    }
}

fn traversal_dfs(adjacency_matrix: &Vec<Vec<i32>>) -> Vec<i32> {
    let n = adjacency_matrix.len();
    let mut seen = vec![false; n];
    let mut values = vec![];
   
    dfs(adjacency_matrix, 0, &mut seen, &mut values);

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

    let result = traversal_dfs(&adjacency_matrix);
    println!("DFS Traversal Result: {:?}", result);
}
