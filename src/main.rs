// src/main.rs
mod euler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a problem name.");
        return;
    }
    euler::run_problem(&args[1]);
}
