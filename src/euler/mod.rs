pub mod problem_001;
pub mod problem_002;
pub mod problem_003;
pub mod problem_004;
pub mod problem_005;
pub mod problem_006;
pub mod problem_007;
// new problem mod






pub fn run_problem(name: &str) {
    match name {
        "problem_001" => problem_001::solve(),
        "problem_002" => problem_002::solve(),
        "problem_003" => problem_003::solve(),
        "problem_004" => problem_004::solve(),
        "problem_005" => problem_005::solve(),
	      "problem_006" => problem_006::solve(),
	      "problem_007" => problem_007::solve(),
        // new problem mapping

        _ => println!("Problem {} not found.", name),
    }
}
