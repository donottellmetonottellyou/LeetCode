/// Constraints:
/// 4 <= numbers.len() <= 10^4
/// 1 <= numbers[i] <= 10^4
pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    /// Panics with numbers.len() < 2. Incorrect with numbers.len() < 4.
    /// Incorrect with numbers[i] < 0.
    pub fn max_product_difference(mut numbers: Vec<i32>) -> i32 {
        // We should never have more than 3 numbers in the heap, because we only
        // need the top two (or bottom two) numbers.
        let mut min_numbers: BinaryHeap<i32> = BinaryHeap::with_capacity(3);
        let mut max_numbers: BinaryHeap<Reverse<i32>> =
            BinaryHeap::with_capacity(3);

        // Fill up the heaps with the first two numbers.
        for _ in 0..2 {
            let number = numbers.pop().unwrap();
            min_numbers.push(number);
            max_numbers.push(Reverse(number));
        }

        // We first push a number onto a heap, then remove the max/min. If we
        // repeat this for all number in numbers, we end up with the maximum two
        // and minimum two numbers.
        for number in numbers.drain(..) {
            min_numbers.push(number);
            min_numbers.pop();

            max_numbers.push(Reverse(number));
            max_numbers.pop();
        }

        // Get the maximum possible and minimum possible products.
        let min_product: i32 = min_numbers.into_iter().product();
        let max_product: i32 =
            max_numbers.into_iter().map(|number| number.0).product();

        max_product - min_product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leet_example_1() {
        let numbers = vec![5, 6, 2, 7, 4];
        assert_eq!(34, Solution::max_product_difference(numbers));
    }

    #[test]
    fn leet_example_2() {
        let numbers = vec![4, 2, 5, 9, 7, 4, 8];
        assert_eq!(64, Solution::max_product_difference(numbers));
    }
}
