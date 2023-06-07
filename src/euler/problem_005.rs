// Project Euler Problem 005:
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

pub fn solve() {
    let smallest = smallest_multiple(20);

    println!("The smallest number that is evenly divisible by all of the numbers from 1 to 20 is {}", smallest)
}

pub fn smallest_multiple(n: i32) -> i32 {
    let mut result = n;
    while !is_evenly_divisible(result, n) {
        result += n;
    }
    result
}

fn is_evenly_divisible(num: i32, n: i32) -> bool {
    for i in 2..=n {
        if num % i != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_smallest_multiple() {
        assert_eq!(smallest_multiple(10), 2520);
        assert_eq!(smallest_multiple(5), 60);
        assert_eq!(smallest_multiple(1), 1);
        assert_eq!(smallest_multiple(2), 2);
    }

    #[test]
    fn test_solve() {
        assert_eq!(smallest_multiple(20), 232792560);
    }
}
