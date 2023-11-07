pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ones = vec![0; n as usize + 1];

        for i in 1..=(n as usize) {
            ones[i] = ones[i >> 1] + (i as i32 & 1);
        }

        ones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static BIT_PATTERN: [i32; 6] = [0, 1, 1, 2, 1, 2];

    #[test]
    fn counts_bits_1() {
        let solution = Vec::from_iter(BIT_PATTERN[0..=2].iter().copied());

        assert_eq!(solution, Solution::count_bits(2));
    }

    #[test]
    fn counts_bits_2() {
        let solution = Vec::from_iter(BIT_PATTERN[0..=5].iter().copied());

        assert_eq!(solution, Solution::count_bits(5));
    }
}
