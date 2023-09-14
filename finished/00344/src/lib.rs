pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        for (i, j) in (0..s.len()).rev().enumerate() {
            if i > j {
                break;
            }

            s.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverses_odd_string() {
        let mut s = vec!['a', 'b', 'c'];
        let result = vec!['c', 'b', 'a'];

        Solution::reverse_string(&mut s);

        assert_eq!(s, result);
    }

    #[test]
    fn reverses_even_string() {
        let mut s = vec!['a', 'b', 'c', 'd'];
        let result = vec!['d', 'c', 'b', 'a'];

        Solution::reverse_string(&mut s);

        assert_eq!(s, result);
    }
}
