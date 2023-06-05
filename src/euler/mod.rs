// src/euler/mod.rs
pub mod problem_001;
pub mod problem_002;
// add other problems here

pub fn run_problem(name: &str) {
    match name {
        "problem_001" => problem_001::solve(),
        "problem_002" => problem_002::solve(),
        // add other problems here
        _ => println!("Problem {} not found.", name),
    }
}
