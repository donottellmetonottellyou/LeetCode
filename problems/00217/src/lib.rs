pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let nums_len = nums.len();

        let mut set = HashSet::with_capacity(nums_len);
        for num in nums {
            set.insert(num);
        }

        set.len() != nums_len
    }
}
