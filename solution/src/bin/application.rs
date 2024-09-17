use std::io::{self, prelude::*};

use todo_swamp::*;

pub fn main() {
    // List for todo
    let mut tl: TodoList = TodoList::new();
    // List for completed Tasks
    let mut cl: Vec<TodoItem> = Vec::new();

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");

    let no_of_queries = input
        .trim()
        .parse::<usize>()
        .expect("Error getting number of queries: invalid input!");

    // Collect all the inputs first
    let mut inputs = Vec::new();
    for line in stdin.lock().lines().take(no_of_queries).flatten() {
        inputs.push(line);
    }

    // Buffer to store the results
    let mut output_buffer = Vec::new();

    // Process all queries in a batch
    for line in inputs {
        if let Some(result) = runner::run_line(&line, &mut tl, &mut cl) {
            output_buffer.push(result);
        }
    }

    // Output results in one go
    for output in output_buffer {
        println!("{}", output);
    }
}
