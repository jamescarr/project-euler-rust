pub mod problem_001;
pub mod problem_002;
pub mod problem_003;
// new problem mod


pub fn run_problem(name: &str) {
    match name {
        "problem_001" => problem_001::solve(),
        "problem_002" => problem_002::solve(),
        "problem_003" => problem_003::solve(),
        // new problem mapping

        _ => println!("Problem {} not found.", name),
    }
}
