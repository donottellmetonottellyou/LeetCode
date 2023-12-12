pub struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;

        for i in 0..arr.len() {
            for j in (i + 1)..arr.len() {
                if (arr[i] - arr[j]).abs() > a {
                    continue;
                }
                for k in (j + 1)..arr.len() {
                    if (arr[j] - arr[k]).abs() <= b
                        && (arr[i] - arr[k]).abs() <= c
                    {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_good_triplets_1() {
        let arr = vec![3, 0, 1, 1, 9, 7];
        let a = 7;
        let b = 2;
        let c = 3;

        assert_eq!(Solution::count_good_triplets(arr, a, b, c), 4);
    }

    #[test]
    fn counts_good_triplets_2() {
        let arr = vec![1, 1, 2, 2, 3];
        let a = 0;
        let b = 0;
        let c = 1;

        assert_eq!(Solution::count_good_triplets(arr, a, b, c), 0);
    }
}
