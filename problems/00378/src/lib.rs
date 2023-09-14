pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap, iter::FromIterator};

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(matrix.into_iter().flatten().map(Reverse));

        for _ in 1..k {
            heap.pop();
        }

        if let Some(Reverse(e)) = heap.pop() {
            e
        } else {
            panic!("k was too large!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_kth_smallest_1() {
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];

        assert_eq!(Solution::kth_smallest(matrix, 8), 13);
    }

    #[test]
    fn finds_kth_smallest_2() {
        let matrix = vec![vec![-5]];

        assert_eq!(Solution::kth_smallest(matrix, 1), -5);
    }

    #[test]
    fn finds_kth_smallest_3() {
        let matrix = vec![vec![1, 2], vec![1, 3]];

        assert_eq!(Solution::kth_smallest(matrix, 2), 1)
    }
}
