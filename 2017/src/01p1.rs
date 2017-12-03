//! Advent of Code 2017 day 1 part 1 <http://adventofcode.com/2017/day/1>
//! 

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdio");

    println!("Got digits: {}", input);
    println!("Sum of digits: {}", sum_of_digits(&input));
}

fn sum_of_digits(digits: &str) -> usize {
    let mut sum: usize = 0;
    let mut chars = digits.trim().chars().chain(digits.chars());
    let mut first = chars.next().expect("first char");

    for _ in 0..digits.len() {
        let second = match chars.next() {
            Some(n) => n,
            None => break,
        };

        if first == second {
            let num: usize = format!("{}", first).parse().expect("parse int");
            sum += num;
        }

        first = second;
    }

    sum
}

#[cfg(test)]
mod tests_01p1 {
    use super::*;

    // 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the
    // second digit and the third digit (2) matches the fourth digit.
    #[test]
    fn sum_of_digits1() {
        let digits = "1122";
        assert_eq!(sum_of_digits(digits), 3);
    }

    // 1111 produces 4 because each digit (all 1) matches the next.
    #[test]
    fn sum_of_digits2() {
        let digits = "1111";
        assert_eq!(sum_of_digits(digits), 4);
    }

    // 1234 produces 0 because no digit matches the next.
    #[test]
    fn sum_of_digits3() {
        let digits = "1234";
        assert_eq!(sum_of_digits(digits), 0);
    }

    // 91212129 produces 9 because the only digit that matches the next one is
    // the last digit, 9.
    #[test]
    fn sum_of_digits4() {
        let digits = "91212129";
        assert_eq!(sum_of_digits(digits), 9);
    }
}
