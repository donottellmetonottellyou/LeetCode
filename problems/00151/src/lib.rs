pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.as_bytes()
            .split(|b| *b == b' ')
            .rev()
            .filter_map(|word| {
                if word.is_empty() {
                    None
                } else {
                    Some(unsafe { String::from_utf8_unchecked(word.to_vec()) })
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverses_words_1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".into()),
            "blue is sky the".to_owned()
        );
    }

    #[test]
    fn reverses_words_2() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".into()),
            "world hello".to_owned()
        );
    }

    #[test]
    fn reverses_words_3() {
        assert_eq!(
            Solution::reverse_words("a good   example".into()),
            "example good a".to_owned()
        );
    }
}
