use adqselect::nth_element;

pub fn get_task(task_id: usize) -> String {
    match std::fs::read_to_string(format!("src/tasks/task{}.txt", task_id)) {
        Ok(x) => x,
        Err(_) => panic!("Error in file fetch."),
    }
}

pub fn mut_median<T: Ord>(s: &mut [T]) -> &mut T {
    let idx = s.len() / 2;
    nth_element(s, idx, &mut Ord::cmp);
    &mut s[idx]
}