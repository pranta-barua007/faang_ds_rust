fn to_adjacency_list(pre_req_list: &Vec<(i32, i32)>, n: usize) -> Vec<Vec<usize>> {
    let mut adjacency_list = vec![Vec::new(); n];
    for &(course, depends_on) in pre_req_list {
        adjacency_list[depends_on as usize].push(course as usize);
    }
    adjacency_list
}

fn can_complete_course_dfs(pre_req_list: Vec<(i32, i32)>, n: i32) -> bool {
    let adj_list = to_adjacency_list(&pre_req_list, n as usize);

    for start in 0..(n as usize) {
        let mut seen = vec![false; n as usize];
        let mut stack = vec![];

        for &neighbor in &adj_list[start] {
            stack.push((neighbor, 0));
        }

        while let Some((current, _)) = stack.pop() {
            if current == start {
                return false; // Cycle detected
            }
            if seen[current] {
                continue;
            }
            seen[current] = true;

            for &next in &adj_list[current] {
                if !seen[next] {
                    stack.push((next, 0));
                }
            }
        }
    }

    true
}

fn main() {
    let pre_req_list = vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)];
    let n = 6;
    let status = can_complete_course_dfs(pre_req_list, n);
    println!("Can complete all courses: {}", status);
}