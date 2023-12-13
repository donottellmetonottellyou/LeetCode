pub struct Solution;

/// Constraints:
/// 1 <= rows, cols <= 100
/// 0 <= r_start < rows
/// 0 <= c_start < cols
impl Solution {
    pub fn spiral_matrix_iii(
        rows: i32,
        columns: i32,
        row: i32,
        column: i32,
    ) -> Vec<Vec<i32>> {
        let mut visited = Vec::with_capacity(rows as usize * columns as usize);
        visited.push(vec![row, column]);

        // Current position.
        let mut position = (row, column);

        // This keeps track of whether we are going in a positive or negative
        // direction.
        let mut direction = 1;

        // The limit increases by one every other step, so we do two steps at
        // per limit.
        for limit in 1.. {
            // =================
            // Break If Finished
            // =================
            if visited.len() == visited.capacity() {
                break;
            }

            // =========
            // East-West
            // =========

            // Skip out-of-bounds steps.
            let mut skip =
                0.min(position.1 + 1).abs().max(position.1 - columns);

            // Skip the whole thing if we are out of bounds in position.0
            // (North-South).
            if !(0..rows).contains(&position.0) {
                skip = limit;
            }

            // Apply skip to position.
            position.1 += skip * direction;

            for i in skip..limit {
                position.1 += direction;

                // If we are out of bounds, skip the rest.
                if !(0..columns).contains(&position.1) {
                    position.1 += (limit - i - 1) * direction;
                    break;
                }

                visited.push(vec![position.0, position.1]);
            }

            // ===========
            // North-South
            // ===========

            // Skip out-of-bounds steps.
            skip = 0.min(position.0 + 1).abs().max(position.0 - rows);

            // Skip the whole thing if we are out of bounds in position.1
            // (East-West).
            if !(0..columns).contains(&position.1) {
                skip = limit;
            }

            // Apply skip.
            position.0 += skip * direction;

            for i in skip..limit {
                position.0 += direction;

                // If we are out of bounds, skip the rest.
                if !(0..rows).contains(&position.0) {
                    position.0 += (limit - i - 1) * direction;
                    break;
                }

                visited.push(vec![position.0, position.1]);
            }

            // ==============
            // Flip Direction
            // ==============
            direction *= -1;
        }

        visited
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_with_leet_example_1() {
        let rows = 1;
        let cols = 4;
        let r_start = 0;
        let c_start = 0;

        let expected = vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]];

        assert_eq!(
            expected,
            Solution::spiral_matrix_iii(rows, cols, r_start, c_start),
        );
    }

    #[test]
    fn correct_with_leet_example_2() {
        let rows = 5;
        let cols = 6;
        let r_start = 1;
        let c_start = 4;

        let expected = vec![
            vec![1, 4],
            vec![1, 5],
            vec![2, 5],
            vec![2, 4],
            vec![2, 3],
            vec![1, 3],
            vec![0, 3],
            vec![0, 4],
            vec![0, 5],
            vec![3, 5],
            vec![3, 4],
            vec![3, 3],
            vec![3, 2],
            vec![2, 2],
            vec![1, 2],
            vec![0, 2],
            vec![4, 5],
            vec![4, 4],
            vec![4, 3],
            vec![4, 2],
            vec![4, 1],
            vec![3, 1],
            vec![2, 1],
            vec![1, 1],
            vec![0, 1],
            vec![4, 0],
            vec![3, 0],
            vec![2, 0],
            vec![1, 0],
            vec![0, 0],
        ];

        assert_eq!(
            expected,
            Solution::spiral_matrix_iii(rows, cols, r_start, c_start),
        );
    }

    /// Order Visited:
    /// 7 8 9
    /// 6 1 2
    /// 5 4 3
    #[test]
    fn visits_all_coordinates_in_a_3x3_matrix() {
        let rows = 3;
        let cols = 3;
        let r_start = 1;
        let c_start = 1;

        let expected = vec![
            vec![1, 1],
            vec![1, 2],
            vec![2, 2],
            vec![2, 1],
            vec![2, 0],
            vec![1, 0],
            vec![0, 0],
            vec![0, 1],
            vec![0, 2],
        ];

        assert_eq!(
            expected,
            Solution::spiral_matrix_iii(rows, cols, r_start, c_start),
        );
    }

    /// Order Visited:
    /// 1 2 5
    /// 4 3 6
    /// 9 8 7
    #[test]
    fn visits_all_coordinates_in_a_3x3_matrix_starting_at_0_0() {
        let rows = 3;
        let cols = 3;
        let r_start = 0;
        let c_start = 0;

        let expected = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 1],
            vec![1, 0],
            vec![0, 2],
            vec![1, 2],
            vec![2, 2],
            vec![2, 1],
            vec![2, 0],
        ];

        assert_eq!(
            expected,
            Solution::spiral_matrix_iii(rows, cols, r_start, c_start),
        );
    }
}
