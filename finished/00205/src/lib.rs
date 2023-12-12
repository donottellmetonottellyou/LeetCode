pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        // We assume s.len() == t.len().

        // Turn s and t into mapped patterns.
        let s = Self::map_bytes(&s);
        let t = Self::map_bytes(&t);

        // map_bytes() will ensure that if s and t are isomorphic, the patterns will be the same.
        s.into_iter().zip(t).all(|(s_byte, t_byte)| s_byte == t_byte)
    }

    fn map_bytes(s: &str) -> Vec<u8> {
        // We turn the &str into a pattern where the first character encountered is 0, the second 1,
        // the third 2, etc. If we find a byte that already was encountered, we use the byte mapping
        // it previously was.

        // Keep track of the nth character
        let mut n: u8 = 0;
        // Keep track of previous characters we have already encountered.
        let mut mappings: HashMap<u8, u8> = HashMap::with_capacity(128);
        let mut mapped: Vec<u8> = Vec::with_capacity(s.len());

        for byte in s.bytes() {
            if let Some(mapped_byte) = mappings.get(&byte) {
                // If we already encountered a byte, push the mapped byte into mapped.
                mapped.push(*mapped_byte);
            } else {
                // If we haven't encountered the byte, record it as the nth byte and increment n.
                mapped.push(n);
                mappings.insert(byte, n);
                n += 1;
            }
        }

        mapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egg_and_add_are_isomorphic() {
        assert!(Solution::is_isomorphic("egg".into(), "add".into()));
    }

    #[test]
    fn foo_and_bar_are_not_isomorphic() {
        assert!(!Solution::is_isomorphic("foo".into(), "bar".into()));
    }

    #[test]
    fn paper_and_title_are_isomorphic() {
        assert!(Solution::is_isomorphic("paper".into(), "title".into()));
    }
}
