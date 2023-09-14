pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut [i32]) {
        if let Some(i) = nums.iter().enumerate().rev().find_map(|(i, e)| {
            // Find the first decrease in nums, and set that to i.
            if let Some(previous) = nums.get(i + 1) {
                if e < previous {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            }
        }) {
            // Find the next largest number's index (the smallest number that is larger than
            // nums[i]) in nums[(i + 1)..] to swap with nums[i] and set it to j, then do the swap.
            let j = nums
                .iter()
                .copied()
                .enumerate()
                .skip(i + 1)
                .fold(
                    i + 1,
                    |j, (k, e)| {
                        if e > nums[i] && e < nums[j] {
                            k
                        } else {
                            j
                        }
                    },
                );
            nums.swap(i, j);

            // Sort nums[(i + 1)..] so that we get the next lexicographical order.
            nums[(i + 1)..].sort();
        } else {
            nums.sort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutes_123() {
        let mut nums = vec![1, 2, 3];
        let solution = vec![1, 3, 2];

        Solution::next_permutation(&mut nums);

        assert_eq!(nums, solution);
    }

    #[test]
    fn permutes_321() {
        let mut nums = vec![3, 2, 1];
        let solution = vec![1, 2, 3];

        Solution::next_permutation(&mut nums);

        assert_eq!(nums, solution);
    }

    #[test]
    fn permutes_115() {
        let mut nums = vec![1, 1, 5];
        let solution = vec![1, 5, 1];

        Solution::next_permutation(&mut nums);

        assert_eq!(nums, solution);
    }
}
