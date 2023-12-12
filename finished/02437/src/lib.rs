pub struct Solution;

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let hour_combinations = match (time.as_bytes()[0], time.as_bytes()[1]) {
            (b'?', b'?') => 24,
            (b'?', b'0'..=b'3') => 3,
            (b'?', _) => 2,
            (b'0'..=b'1', b'?') => 10,
            (_, b'?') => 4,
            (_, _) => 1,
        };
        let minute_combinations = match (time.as_bytes()[3], time.as_bytes()[4])
        {
            (b'?', b'?') => 60,
            (b'?', _) => 6,
            (_, b'?') => 10,
            (_, _) => 1,
        };

        hour_combinations * minute_combinations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_time_1() {
        assert_eq!(Solution::count_time("?5:00".into()), 2);
    }

    #[test]
    fn counts_time_2() {
        assert_eq!(Solution::count_time("0?:0?".into()), 100);
    }

    #[test]
    fn counts_time_3() {
        assert_eq!(Solution::count_time("??:??".into()), 1440);
    }
}
