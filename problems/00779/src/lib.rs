pub struct Solution;

impl Solution {
    /// Every iteration is the previous iteration + the complement, so we only have to worry about
    /// how to generate the sequence to the kth element:
    ///
    /// 0 -> 0|1 -> 01|10 -> 0110|1001 -> 01101001|10010110
    ///
    /// An easy way to generate the kth element according to
    /// [here](https://en.wikipedia.org/wiki/Thue%E2%80%93Morse_sequence#Direct_definition)
    /// (starting from zero) is to look at the number k in binary and count the ones. If the number
    /// of ones is odd, then the nth element is 1, otherwise it is 0.
    pub fn kth_grammar(_n: i32, mut k: i32) -> i32 {
        //  Start k at zero:
        k -= 1;
        //  If k's ones are evenly counted, the kth element is 1
        Self::count_ones(k) % 2
    }

    fn count_ones(mut k: i32) -> i32 {
        let mut count = 0;

        for _ in 0..32 {
            //  Count 1 at bottom.
            count += k % 2;
            //  Go to next digit.
            k /= 2;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_kth_grammar_1() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
    }

    #[test]
    fn finds_kth_grammar_2() {
        assert_eq!(Solution::kth_grammar(2, 1), 0);
    }

    #[test]
    fn finds_kth_grammar_3() {
        assert_eq!(Solution::kth_grammar(2, 2), 1);
    }

    #[test]
    fn finds_kth_grammar_4() {
        assert_eq!(Solution::kth_grammar(30, 178490340), 1);
    }
}
