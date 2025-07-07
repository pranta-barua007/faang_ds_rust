fn to_adjacency_list(managers: &Vec<i32>) -> Vec<Vec<usize>> {
    let n = managers.len();
    let mut adjacency_list = vec![Vec::new(); n];

    for (employee, &manager) in managers.iter().enumerate() {
        if manager != -1 {
            adjacency_list[manager as usize].push(employee);
        }
    }

    adjacency_list
}

fn dfs_iterative(current_id: usize, adjacency_list: &Vec<Vec<usize>>, inform_time: &Vec<i32>) -> i32 {
    let mut stack = vec![(current_id, 0)];
    let mut max_time = 0;

    while let Some((current_id, current_time )) = stack.pop() {
        max_time = max_time.max(current_time);

        for &employee_id in &adjacency_list[current_id] {
            stack.push((employee_id, current_time + inform_time[current_id]));
        }
    }

    max_time
}

fn num_of_minutes(head_id: i32, managers: Vec<i32>, inform_time: Vec<i32>) -> i32 {
   let adjacency_list = to_adjacency_list(&managers);
   return dfs_iterative(head_id as usize, &adjacency_list, &inform_time);
}

fn main() {
   let managers_list = vec![2, 2, 4, 6, -1, 4, 4, 5];
   let inform_time_list = vec![0, 0, 4, 0, 7, 3, 6, 0];
   let num_of_minutes_result = num_of_minutes(
       4,
       managers_list,
       inform_time_list
   );
   print!("Number of minutes to inform all employees: {}\n", num_of_minutes_result);
}
