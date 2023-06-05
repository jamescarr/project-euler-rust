# Project Euler: Rust Edition

Early in my career I learned so much by doing project euler problems in ruby. Given it is about 12 year later,
I am embarking on another journey to do [Project Euler](https://projecteuler.net/) problems in Rust!

## Running These Examples

Run an individual problem solution via the make target that contains the problem number.

```bash
$ make problem_001
The sum of multiples of 3 and 5 below 1000 is: 233168
problem_001 completed successfully.

```

## Running the Tests

There are two make targets for running tests.

- `run_tests` - Runs all tests in the project
- `run_tests_<problem number>` - Runs all tests for the specified problem.

For example, to only run the tests for problem 001:

```bash
$ make run_tests_001
Running tests for problem_001 ...
   Compiling project_euler-rust v0.1.0 (/Users/jamescarr/projects/rust/project-euler-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.81s
     Running unittests src/main.rs (target/debug/deps/project_euler_rust-cd01e5e5c30bddb0)

running 2 tests
test euler::problem_001::tests::test_sum_of_multiples_10 ... ok
test euler::problem_001::tests::test_sum_of_multiples_1000 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
