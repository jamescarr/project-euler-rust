// src/euler/mod.rs
pub mod problem_001;
// add other problems here

pub fn run_problem(name: &str) {
    match name {
        "problem_001" => problem_001::problem_001(),
        // add other problems here
        _ => println!("Problem {} not found.", name),
    }
}
