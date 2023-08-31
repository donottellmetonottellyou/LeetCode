pub struct Solution;

use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // Create a map of all the strings that match at a given location.
        let mut words_found: HashMap<usize, Vec<&str>> = HashMap::new();
        for word in &word_dict {
            for i in Self::find_all(&s, word) {
                words_found
                    .entry(i)
                    .and_modify(|words| {
                        words.push(word);
                    })
                    .or_insert(vec![word]);
            }
        }

        // Create an ordered set starting with the location of each word match's end.
        let mut word_progresses = BTreeSet::new();
        if let Some(starting_words) = words_found.get(&0) {
            for word in starting_words {
                word_progresses.insert(word.len());
            }
        }

        // Starting with the smallest indexes, see if there is a matching word at the end of the
        // previous word. We do the smallest first, to deduplicate multiple ways to reach larger
        // indexes. For compatability with leetcode's old rust version, we don't use pop_first().
        while let Some(word_progress) = word_progresses.iter().nth(0) {
            let word_progress = *word_progress;
            word_progresses.remove(&word_progress);

            // If we are at the end of the word, we are finished.
            if word_progress == s.len() {
                return true;
            }

            // For each word we find at the index, push the new index where that word ends.
            if let Some(next_words) = words_found.get(&word_progress) {
                for next_word in next_words.iter() {
                    word_progresses.insert(word_progress + next_word.len());
                }
            }
        }

        false
    }

    fn find_all(mut s: &str, word: &str) -> Vec<usize> {
        let mut all = Vec::new();
        // Keep track of where the original start of s was.
        let mut offset = 0;

        while let Some(i) = s.find(word) {
            all.push(i + offset);

            // To get the next instance of word in s, add 1 to the number where the previous word
            // was found.
            let next = i + 1;
            // Keep track of where the actual i is.
            offset += next;
            // Make s point to a new str with the chars up to next chopped off.
            s = &s[next..];
        }

        all
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_leetcode() {
        assert!(Solution::word_break(
            "leetcode".into(),
            vec!["leet".into(), "code".into()],
        ));
    }

    #[test]
    fn matches_applepenapple() {
        assert!(Solution::word_break(
            "applepenapple".into(),
            vec!["apple".into(), "pen".into()]
        ));
    }

    #[test]
    fn does_not_match_catsandog() {
        assert!(!Solution::word_break(
            "catsandog".into(),
            vec![
                "cats".into(),
                "dog".into(),
                "sand".into(),
                "and".into(),
                "cat".into()
            ]
        ))
    }

    #[test]
    fn matches_aaaaaaa() {
        assert!(Solution::word_break(
            "aaaaaaa".into(),
            vec!["aaaa".into(), "aaa".into()]
        ))
    }

    #[test]
    fn super_aaa() {
        // Takes too long + uses too much memory.
        let aaa = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            aaaaaaaaaaaaaaaaaaaaaaab"
            .into();
        assert!(!Solution::word_break(
            aaa,
            vec![
                "a",
                "aa",
                "aaa",
                "aaaa",
                "aaaaa",
                "aaaaaa",
                "aaaaaaa",
                "aaaaaaaa",
                "aaaaaaaaa",
                "aaaaaaaaaa"
            ]
            .into_iter()
            .map(|s| s.into())
            .collect()
        ))
    }
}
