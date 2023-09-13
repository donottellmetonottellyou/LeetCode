pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        //  Base case: arrays with only one element.
        if nums.len() == 1 {
            return nums[0];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        //  Base case: a sorted array will have the lowest at the bottom.
        if nums[left] < nums[right] {
            return nums[left];
        }

        //  Binary search for the inflection point.
        while left != right - 1 {
            let middle = {
                let sum = left + right;
                sum / 2 + sum % 2
            };
            if nums[middle] <= nums[right] {
                right = middle;
            } else {
                left = middle;
            }
        }

        //  At this point, the left should point at the highest value and the right should point at
        //  the lowest.
        nums[right]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_min_1() {
        let nums = vec![3, 4, 5, 1, 2];

        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn finds_min_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];

        assert_eq!(Solution::find_min(nums), 0);
    }

    #[test]
    fn finds_min_3() {
        let nums = vec![11, 13, 15, 17];

        assert_eq!(Solution::find_min(nums), 11);
    }

    #[test]
    fn finds_min_4() {
        let nums = vec![5, 1, 2, 3, 4];

        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn finds_min_5() {
        let nums = vec![2, 3, 4, 5, 1];

        assert_eq!(Solution::find_min(nums), 1)
    }

    #[test]
    fn finds_min_6() {
        let nums = vec![1];

        assert_eq!(Solution::find_min(nums), 1)
    }
}
