pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // If s & t could contain unicode, we would have to use char's. Alternatively for more
        // accuracy, we would import a crate to separate graphemes. If s & t could be longer, we
        // would have to use a u32 or u64 for the counts.

        // Turn s into counts of s' bytes.
        let mut s_counts: HashMap<u8, u16> = HashMap::new();
        for byte in s.into_bytes() {
            s_counts.entry(byte).and_modify(|c| *c += 1).or_insert(1);
        }

        // Turn t into counts of t's bytes.
        let mut t_counts: HashMap<u8, u16> = HashMap::new();
        for byte in t.into_bytes() {
            t_counts.entry(byte).and_modify(|c| *c += 1).or_insert(1);
        }

        s_counts == t_counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anagram_nagaram_are_anagrams() {
        assert!(Solution::is_anagram("anagram".into(), "nagaram".into()));
    }

    #[test]
    fn rat_car_are_not_anagrams() {
        assert!(!Solution::is_anagram("rat".into(), "car".into()));
    }
}
