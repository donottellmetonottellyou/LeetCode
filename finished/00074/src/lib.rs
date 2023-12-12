pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // Search rows by the last element in the row, if found return true. If Err, the value
        // contained will tell us the row where the element to be found would be in.
        match matrix.binary_search_by_key(&target, |row| *row.last().unwrap()) {
            Ok(_) => true,
            // Search the row given by Err() for the value.
            Err(i) => {
                if i < matrix.len() {
                    matrix[i].binary_search(&target).is_ok()
                } else {
                    // If i >= matrix.len(), it is out of bounds and there is nothing to search.
                    false
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_matrix_tests_true() {
        let matrix =
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(Solution::search_matrix(matrix, 3));
    }

    #[test]
    fn second_matrix_tests_false() {
        let matrix =
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(!Solution::search_matrix(matrix, 13));
    }

    #[test]
    fn third_matrix_tests_false() {
        let matrix = vec![vec![1]];
        assert!(!Solution::search_matrix(matrix, 2));
    }
}
