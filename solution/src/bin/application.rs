use std::io::{self, prelude::*};
use std::time::Instant;

use todo_swamp::*;

pub fn main() {
    let start_main = Instant::now();

    let start = Instant::now();
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");

    let no_of_queries = input
        .trim()
        .parse::<usize>()
        .expect("Error getting number of queries: invalid input!");

    let mut tl: TodoList = TodoList::new(no_of_queries);

    // Collect all the inputs first
    let mut inputs = Vec::with_capacity(no_of_queries);
    for line in stdin.lock().lines().take(no_of_queries).flatten() {
        inputs.push(line);
    }
    let duration = start.elapsed();
    println!("{}", &format!("Time taken for read: {:?}", duration));

    let start = Instant::now();
    // Buffer to store the results
    let mut output_buffer = Vec::new();

    // Process all queries in a batch
    for line in inputs {
        if let Some(result) = runner::run_line(&line, &mut tl) {
            output_buffer.push(result);
        }
    }
    let duration = start.elapsed();
    println!("{}", &format!("Time taken for process: {:?}", duration));

    let start = Instant::now();
    // Output results in one go
    for output in output_buffer {
        println!("{}", output);
    }
    let duration = start.elapsed();
    println!("{}", &format!("Time taken for write: {:?}", duration));

    let duration_main = start_main.elapsed();
    println!("{}", &format!("Time taken for main: {:?}", duration_main));
}
