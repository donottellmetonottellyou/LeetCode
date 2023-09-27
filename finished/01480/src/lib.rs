pub struct Solution;

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1]
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_running_sum_1() {
        let nums = vec![1, 2, 3, 4];
        let solution = vec![1, 3, 6, 10];

        assert_eq!(Solution::running_sum(nums), solution);
    }

    #[test]
    fn gets_running_sum_2() {
        let nums = vec![1, 1, 1, 1, 1];
        let solution = vec![1, 2, 3, 4, 5];

        assert_eq!(Solution::running_sum(nums), solution);
    }

    #[test]
    fn gets_running_sum_3() {
        let nums = vec![3, 1, 2, 10, 1];
        let solution = vec![3, 4, 6, 16, 17];

        assert_eq!(Solution::running_sum(nums), solution);
    }
}
