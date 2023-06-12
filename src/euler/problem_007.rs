// Problem 007 
// By listing the first six prime numbers: 2, 3, 5, 7, 11, 
// and 13, we can see that the 6th prime is 13. What is the 10,000th prime number?
use colored::*;

pub fn solve() {
    let result = nth_prime(10_000);

    colorful_print("The 10,000th prime number is", result);
}

fn colorful_print(text: &str, result: u32) {
    println!("{} {}", 
        text.green(),
        result.to_string().yellow().on_blue().bold());
}

fn nth_prime(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut i = 2;

    while (primes.len() as u32) < n {
        if primes.iter().all(|p| i % p != 0) {
            primes.push(i);
        }
        i += 1;
    }

    *primes.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_prime_6() {
        assert_eq!(nth_prime(6), 13);
    }

    #[test]
    fn test_nth_prime_1() {
        assert_eq!(nth_prime(1), 2);
    }

    #[test]
    fn test_nth_prime_2() {
        assert_eq!(nth_prime(2), 3);
    }

    #[test]
    fn test_nth_prime_3() {
        assert_eq!(nth_prime(3), 5);
    }

    #[test]
    fn test_nth_prime_10000() {
        assert_eq!(nth_prime(10000), 104729);
    }
}
