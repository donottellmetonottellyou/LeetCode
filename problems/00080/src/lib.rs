pub struct Solution;

impl Solution {
    /// This algorithm removes duplicates as follows, with x representing ignored values:
    /// 1 1 1 2 2 2 3 3
    /// 1 1 x 2 2 2 3 3
    /// 1 1 2 x 2 2 3 3
    /// 1 1 2 2 x 2 3 3
    /// 1 1 2 2 x x 3 3
    /// 1 1 2 2 3 x x 3
    /// 1 1 2 2 3 3 x x
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut already_double = if let Some(num2) = nums.get(1) {
            nums[0] == *num2
        } else {
            return 1;
        };
        let mut target = 2;

        for i in 2..nums.len() {
            if nums[i - 1] == nums[i] {
                if already_double {
                    continue;
                } else {
                    already_double = true;

                    nums[target] = nums[i];
                    target += 1;
                }
            } else {
                already_double = false;

                nums[target] = nums[i];
                target += 1;
            }
        }

        target as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_duplicates_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];

        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[0..5], [1, 1, 2, 2, 3]);
    }

    #[test]
    fn removes_duplicates_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];

        assert_eq!(Solution::remove_duplicates(&mut nums), 7);
        assert_eq!(nums[0..7], [0, 0, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn removes_duplicates_3() {
        let mut nums = vec![1];

        assert_eq!(Solution::remove_duplicates(&mut nums), 1);
        assert_eq!(nums[..], [1]);
    }

    #[test]
    fn removes_duplicates_4() {
        let mut nums = vec![1, 1, 1, 2, 2, 2, 3, 3];

        assert_eq!(Solution::remove_duplicates(&mut nums), 6);
        assert_eq!(nums[0..6], [1, 1, 2, 2, 3, 3]);
    }
}
