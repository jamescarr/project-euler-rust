// Problem 1: Multiples of 3 and 5
//
//  If we list all the natural numbers below 10 that are multiples of 3 or 5, we get
//  3, 5, 6 and 9. The sum of these multiples is 23.
//
//  Find the sum for multiples of 3 and 5 below 1000.
pub fn problem_001() {
    let target_number = 1000;
    let result = sum_of_multiples(target_number);
    println!("The sum of multiples of 3 and 5 below {} is: {}", target_number, result);
}

fn sum_of_multiples(target: u32) -> u32 {
    let mut sum = 0;
    for num in 1..target {
        if num % 3 == 0 || num % 5 == 0 {
            sum += num;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_multiples_10() {
        let target = 10;
        let expected = 23;  // 3 + 5 + 6 + 9 = 23
        assert_eq!(sum_of_multiples(target), expected);
    }

    #[test]
    fn test_sum_of_multiples_1000() {
        let target = 1000;
        let expected = 233_168;
        assert_eq!(sum_of_multiples(target), expected);
    }
}
