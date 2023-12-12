pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = Vec::with_capacity(n as usize);

        for n in 1..=n {
            // At most will contain "FizzBuzz", which is 8 bytes long, unless n > 99999999.
            let mut string = String::with_capacity(8);

            if n % 3 == 0 {
                string.push_str("Fizz");
            }
            if n % 5 == 0 {
                string.push_str("Buzz");
            }
            if string.is_empty() {
                string.push_str(&n.to_string())
            }

            answer.push(string);
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_owned_strings(slices: Vec<&str>) -> Vec<String> {
        slices.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn fizz_buzzes_1() {
        let solution = to_owned_strings(vec!["1", "2", "Fizz"]);

        assert_eq!(solution, Solution::fizz_buzz(3));
    }

    #[test]
    fn fizz_buzzes_2() {
        let solution = to_owned_strings(vec!["1", "2", "Fizz", "4", "Buzz"]);

        assert_eq!(solution, Solution::fizz_buzz(5));
    }

    #[test]
    fn fizz_buzzes_3() {
        let solution = to_owned_strings(vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz",
            "11", "Fizz", "13", "14", "FizzBuzz",
        ]);

        assert_eq!(solution, Solution::fizz_buzz(15));
    }
}
