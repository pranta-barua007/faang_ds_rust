fn dfs(adjacency_list: &Vec<Vec<usize>>, node: usize, visited: &mut Vec<bool>, result: &mut Vec<usize>) {
    visited[node] = true;
    result.push(node);

    for &neighbor in &adjacency_list[node] {
        if !visited[neighbor] {
            dfs(adjacency_list, neighbor, visited, result);
        }
    }
}

fn traversal_dfs(adjacency_list: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited = vec![false; adjacency_list.len()];
    let mut result = Vec::new();

    dfs(adjacency_list, 0, &mut visited, &mut result);

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

    let result = traversal_dfs(&adjacency_list);
    println!("DFS Traversal Result: {:?}", result);
}
