pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Count the number of occurences of each number and store it in a hashmap such that num =>
        // count.
        let mut num_counts: HashMap<i32, u32> = HashMap::new();
        for num in nums {
            num_counts.entry(num).and_modify(|c| *c += 1).or_insert(1);
        }

        // Push each number into a heap, removing the smallest counts when count_priority > k.
        let mut count_priority = BinaryHeap::with_capacity(k as usize + 1);
        for (num, count) in num_counts {
            count_priority.push((Reverse(count), num));

            if count_priority.len() > k as usize {
                count_priority.pop();
            }
        }

        // Remove the count from the heap and collect into a Vec.
        count_priority
            .into_iter()
            .map(|(_count, num)| num)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn finds_top_2_in_111223() {
        let nums = vec![1, 1, 1, 2, 2, 3];

        let top_2: HashSet<i32> =
            HashSet::from_iter(Solution::top_k_frequent(nums, 2));

        assert_eq!(top_2, HashSet::from([1, 2]));
    }

    #[test]
    fn finds_top_1_in_1() {
        let nums = vec![1];

        let top_1: HashSet<i32> =
            HashSet::from_iter(Solution::top_k_frequent(nums, 1));

        assert_eq!(top_1, HashSet::from([1]));
    }
}
