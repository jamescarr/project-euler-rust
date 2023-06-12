// Problem 006:
// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 552 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers 
// and the square of the sum is 3025 − 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred natural numbers 
// and the square of the sum.

pub fn solve() {
    let result = sum_square_difference(100);

    println!("The difference between the sum of the squares of the first one hundred natural numbers and the square of the sum is {}", result);
}

fn sum_square_difference(n: u32) -> u32 {
    let sum_of_squares = (1..=n).map(|i| i * i).sum::<u32>();
    let square_of_sum = (1..=n).sum::<u32>().pow(2);
    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_square_difference_10() {
        assert_eq!(sum_square_difference(10), 2640);
    }

    #[test]
    fn test_sum_square_difference_5() {
        assert_eq!(sum_square_difference(5), 170);
    }

    #[test]
    fn test_sum_square_difference_2() {
        assert_eq!(sum_square_difference(2), 4);
    }

    #[test]
    fn test_sum_square_difference_1() {
        assert_eq!(sum_square_difference(1), 0);
    }

    #[test]
    fn test_sum_square_difference_100() {
        assert_eq!(sum_square_difference(100), 25164150);
    }
}
