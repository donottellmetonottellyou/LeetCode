pub struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let prefix_sums = Self::calculate_prefix_sums(arr);

        queries
            .into_iter()
            .map(|query| (query[0] as usize, query[1] as usize + 1))
            .map(|(left_bits, right_bits)| {
                prefix_sums[left_bits] ^ prefix_sums[right_bits]
            })
            .collect()
    }

    fn calculate_prefix_sums(arr: Vec<i32>) -> Vec<i32> {
        let mut prefix_sums = Vec::with_capacity(arr.len() + 1);
        prefix_sums.push(0);

        for num in arr {
            prefix_sums.push(prefix_sums.last().unwrap() ^ num);
        }

        prefix_sums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_queries_1() {
        let arr = vec![1, 3, 4, 8];
        let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];

        assert_eq!(Solution::xor_queries(arr, queries), vec![2, 7, 14, 8]);
    }

    #[test]
    fn xor_queries_2() {
        let arr = vec![4, 8, 2, 10];
        let queries = vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]];

        assert_eq!(Solution::xor_queries(arr, queries), vec![8, 0, 4, 4]);
    }
}
