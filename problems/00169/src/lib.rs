pub struct Solution;

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut streak = 0;
        let mut current = nums[0];
        let nums_len = nums.len();
        for num in nums {
            if nums_len <= streak * 2 {
                return current;
            }

            if num == current {
                streak += 1;
            } else {
                current = num;
                streak = 1;
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 2, 3];
        assert_eq!(Solution::majority_element(nums), 3);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(Solution::majority_element(nums), 2);
    }
}
