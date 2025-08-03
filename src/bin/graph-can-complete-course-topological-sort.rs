use std::vec;

fn to_adjacency_and_indegree_list(pre_req_list: &Vec<(i32, i32)>, n: usize) -> (Vec<Vec<usize>>, Vec<usize>) {
    let mut adjacency_list = vec![Vec::new(); n];
    let mut indegree_list = vec![0; n];

    for &(course, depends_on) in pre_req_list {
        let course_usize = course as usize;
        let depends_on_usize = depends_on as usize;

        adjacency_list[depends_on_usize].push(course_usize);
        indegree_list[course_usize] += 1;
    }

    (adjacency_list, indegree_list)
}

// Time Complexity: O(P + n^2) where P is the number of prerequisites and n is the number of courses.
// Space Complexity: O(n^2) 
fn can_complete_course_dfs(pre_req_list: Vec<(i32, i32)>, n: i32) -> bool {
    let (adj_list, mut indegree_list) = to_adjacency_and_indegree_list(&pre_req_list, n as usize);
    let mut stack = vec![];

    for item in 0..indegree_list.len() {
        if indegree_list[item] == 0 {
            stack.push(item);
        }
    }

    let mut count = 0;

    while !stack.is_empty() {
        let course = stack.pop().unwrap();
        count += 1;

        for &next in &adj_list[course] {
            if indegree_list[next] > 0 {
                indegree_list[next] -= 1;
                if indegree_list[next] == 0 {
                    stack.push(next);
                }
            }
        }
    }

    count == n as usize
}

fn main() {
    let pre_req_list = vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)];
    let n = 6;
    let status = can_complete_course_dfs(pre_req_list, n);
    println!("Can complete all courses: {}", status);
}