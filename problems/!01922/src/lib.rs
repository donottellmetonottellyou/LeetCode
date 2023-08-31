pub struct Solution;

impl Solution {
    const ANSWER_MODULO: u128 = 1_000_000_007;
    const SIXTEEN_STEPPED: u128 = 4_u128.pow(16) * 5_u128.pow(16);

    pub fn count_good_numbers(mut n: i64) -> i32 {
        let mut good_numbers: u128 = 1;

        loop {
            // 16x even + 16x odd
            n -= 32;
            if n < 0 {
                n += 32;
                break;
            }

            good_numbers *= Self::SIXTEEN_STEPPED;
            good_numbers %= Self::ANSWER_MODULO;
        }

        for _even in (0..n).step_by(2) {
            // Possible = 0, 2, 4, 6, 8
            good_numbers *= 5;
            good_numbers %= Self::ANSWER_MODULO;
        }

        for _odd in (1..n).step_by(2) {
            // Possible = 2, 3, 5, 7
            good_numbers *= 4;
            good_numbers %= Self::ANSWER_MODULO;
        }

        good_numbers as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_good_numbers_with_length_1() {
        assert_eq!(Solution::count_good_numbers(1), 5);
    }

    #[test]
    fn counts_good_numbers_with_length_4() {
        assert_eq!(Solution::count_good_numbers(4), 400);
    }

    #[test]
    fn counts_good_numbers_with_length_50() {
        assert_eq!(Solution::count_good_numbers(50), 564908303);
    }

    // #[test]
    // fn counts_good_numbers_with_length_806166225460393() {
    //     // Too slow!
    //     dbg!(Solution::count_good_numbers(806166225460393));
    // }
}
