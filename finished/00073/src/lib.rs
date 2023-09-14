pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        // Check if the first row/column should be set to zero.
        let first_row_zero = matrix[0].iter().any(|e| *e == 0);
        let first_column_zero = matrix.iter().any(|row| row[0] == 0);

        // ===========================================================
        // For all of the next three loops, skip the first row/column.
        // ===========================================================

        // Make the first element in each row/column of a zero into zero.
        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // Now if we find a zero in the beginning of a row, set it all to zero.
        for row in matrix.iter_mut().skip(1) {
            if row[0] == 0 {
                for e in row {
                    *e = 0;
                }
            }
        }

        // If we find a zero in the first row, set the column to zero.
        for row in matrix.iter() {
            println!("{row:?}");
        }
        for j in 1..matrix[0].len() {
            if matrix[0][j] == 0 {
                for row in matrix.iter_mut() {
                    row[j] = 0;
                }
            }
        }

        // ===================================
        // Finally we do the first row/column.
        // ===================================

        if first_row_zero {
            for e in &mut matrix[0] {
                *e = 0;
            }
        }

        if first_column_zero {
            for row in matrix.iter_mut() {
                row[0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_zeroes_1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let solution = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];

        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, solution);
    }

    #[test]
    fn sets_zeroes_2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let solution = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];

        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, solution);
    }

    #[test]
    fn sets_zeroes_3() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![0, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];
        let solution = vec![
            vec![0, 0, 3, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        Solution::set_zeroes(&mut matrix);

        assert_eq!(matrix, solution);
    }
}
