pub fn get_task(task_id: usize) -> String {
    match std::fs::read_to_string(format!("src/tasks/task{}.txt", task_id)) {
        Ok(x) => x,
        Err(_) => panic!("Error in file fetch.")
    }
}