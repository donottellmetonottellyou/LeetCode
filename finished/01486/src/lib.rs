pub struct Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut next = start;
        let mut xor_target = start;

        for _ in 1..n {
            next += 2;
            xor_target ^= next;
        }

        xor_target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_operation_1() {
        assert_eq!(Solution::xor_operation(5, 0), 8);
    }

    #[test]
    fn xor_operation_2() {
        assert_eq!(Solution::xor_operation(4, 3), 8);
    }

    #[test]
    fn xor_operation_3() {
        assert_eq!(Solution::xor_operation(1, 0), 0);
    }
}
