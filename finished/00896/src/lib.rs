pub struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    /// Solution to whether a vec is monotonic.
    /// This function assumes:
    /// 1 <= nums.length <= 100_000
    /// -100_000 <= nums[i] <= 100_000
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        // Some(true) is increasing, Some(false) is decreasing, None is undetermined.
        let mut monotonic_increasing = None;

        nums.windows(2).all(|window| {
            match window[0].cmp(&window[1]) {
                // If there was an increase...
                Less => match monotonic_increasing {
                    // ...and it was either undetermined or increasing, set it to Some(true),...
                    None | Some(true) => monotonic_increasing = Some(true),
                    //  ...or else it wasn't monotonic.
                    Some(false) => return false,
                },
                // Otherwise, if there was a decrease...
                Greater => match monotonic_increasing {
                    // ...and it was either undetermined or decreasing, set it to Some(false),...
                    None | Some(false) => monotonic_increasing = Some(false),
                    // ...or else it wasn't monotonic.
                    Some(true) => return false,
                },
                Equal => {}
            }
            true
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_is_monotonic_1() {
        let nums = vec![1, 2, 2, 3];

        assert!(Solution::is_monotonic(nums));
    }

    #[test]
    fn finds_is_monotonic_2() {
        let nums = vec![6, 5, 4, 4];

        assert!(Solution::is_monotonic(nums));
    }

    #[test]
    fn finds_is_not_monotonic_1() {
        let nums = vec![1, 3, 2];

        assert!(!Solution::is_monotonic(nums));
    }
}
