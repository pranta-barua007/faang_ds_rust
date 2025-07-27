use std::collections::VecDeque;

fn to_adjacency_list(pre_req_list: &Vec<(i32, i32)>, n: usize) -> Vec<Vec<usize>> {
    let mut adjacency_list = vec![Vec::new(); n];
    for &(course, depends_on) in pre_req_list {
        adjacency_list[depends_on as usize].push(course as usize);
    }
    adjacency_list
}

// Time Complexity: O(P + n^3) where P is the number of prerequisites and n is the number of courses.
// Space Complexity: O(n^2 + 2n) 
// for space complexit 2n -> (seen + queue); as we alread have n^2 so we dro 2n
// Final Space Complexity: O(n^2)
fn can_complete_course_bfs(pre_req_list: Vec<(i32, i32)>, n: i32) -> bool {
    let adj_list = to_adjacency_list(&pre_req_list, n as usize);

    for start in 0..(n as usize) {
        let mut seen = vec![false; n as usize];
        let mut queue = VecDeque::new();

        for &neighbor in &adj_list[start] {
            queue.push_back(neighbor);
        }

        while let Some(current) = queue.pop_front() {
            if current == start {
                return false; // Cycle detected
            }
            seen[current] = true;

            for &next in &adj_list[current] {
                if !seen[next] {
                    queue.push_back(next);
                }
            }
        }
    }

    true
}

fn main() {
    // its not gurranteed that all nodes will be connected
    // tuple consist (course, depends_on) ex: (1, 0) i.e. course 0 must be completed for course 1
    
    let pre_req_list = vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)];
    let n = 6;
    let status = can_complete_course_bfs(pre_req_list, n);
    println!("Can complete all courses: {}", status);
}