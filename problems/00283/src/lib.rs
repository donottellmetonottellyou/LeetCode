pub struct Solution;

impl Solution {
    pub fn move_zeroes(mut nums: &mut [i32]) {
        let mut zero_skip = 1;
        // While there are still zeros...
        while let Some(zero_i) =
            nums.iter()
                .enumerate()
                .find_map(|(i, e)| if *e == 0 { Some(i) } else { None })
        {
            // And there is a number to swap...
            if let Some(other_i) = nums
                .iter()
                .enumerate()
                .skip(zero_i + zero_skip)
                .find_map(|(i, e)| if *e != 0 { Some(i) } else { None })
            {
                // Record where the last zero will be...
                zero_skip = other_i - zero_i;
                // Swap that number...
                nums.swap(zero_i, other_i);
                // And reduce the problem.
                nums = &mut nums[(zero_i + 1)..];
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_zeroes_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let solution = vec![1, 3, 12, 0, 0];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, solution);
    }

    #[test]
    fn moves_zeroes_2() {
        let mut nums = vec![0];
        let solution = vec![0];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, solution);
    }

    #[test]
    fn moves_zeroes_3() {
        let mut nums = vec![4, 2, 4, 0, 0, 3, 0, 5, 1, 0];
        let solution = vec![4, 2, 4, 3, 5, 1, 0, 0, 0, 0];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, solution);
    }

    #[test]
    fn moves_zeroes_4() {
        let mut nums = vec![
            -58305, -22022, 0, 0, 0, 0, 0, -76070, 42820, 48119, 0, 95708, -91393, 60044, 0,
            -34562, 0, -88974,
        ];
        let solution = vec![
            -58305, -22022, -76070, 42820, 48119, 95708, -91393, 60044, -34562, -88974, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, solution);
    }

    #[test]
    fn moves_zeroes_5() {
        let mut nums = vec![
            0, 7718, 0, 66675, -43904, -52966, 0, -80723, 0, 17547, -15629, -39356, 90508, 85301,
            -54981, 0, 67353, -17090, -42872, 0, 23726, 0, 0, 0, -5570, -68946, 0, 0, -2184,
            -14438,
        ];
        let solution = vec![
            7718, 66675, -43904, -52966, -80723, 17547, -15629, -39356, 90508, 85301, -54981,
            67353, -17090, -42872, 23726, -5570, -68946, -2184, -14438, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ];

        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, solution);
    }
}
