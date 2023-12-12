pub struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        if nums.len() % 2 == 1 {
            return false;
        }

        let mut not_paired = Vec::new();
        for num in nums {
            if let Some(unpaired_at) =
                not_paired.iter().position(|unpaired| *unpaired == num)
            {
                not_paired.remove(unpaired_at);
            } else {
                not_paired.push(num);
            }
        }

        not_paired.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_array_divisible_1() {
        let nums = vec![3, 2, 3, 2, 2, 2];

        assert!(Solution::divide_array(nums));
    }

    #[test]
    fn finds_array_divisible_2() {
        let nums = vec![
            9, 4, 18, 3, 2, 6, 18, 15, 7, 15, 6, 4, 15, 14, 7, 4, 15, 4, 3, 17,
            9, 13, 13, 12, 2, 14, 12, 17,
        ];

        assert!(Solution::divide_array(nums));
    }

    #[test]
    fn finds_array_indivisible_1() {
        let nums = vec![1, 2, 3, 4];

        assert!(!Solution::divide_array(nums));
    }
}
