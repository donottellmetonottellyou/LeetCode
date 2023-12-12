pub struct Solution;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let mut x = 0;
        let mut y = 0;
        let mut direction = Direction::Right;

        for num in 1..=n * n {
            matrix[x][y] = num;
            match direction {
                Direction::Right => {
                    if y + 1 < n as usize && matrix[x][y + 1] == 0 {
                        y += 1;
                    } else {
                        direction = Direction::Down;
                        x += 1;
                    }
                }
                Direction::Down => {
                    if x + 1 < n as usize && matrix[x + 1][y] == 0 {
                        x += 1;
                    } else {
                        direction = Direction::Left;
                        y -= 1;
                    }
                }
                Direction::Left => {
                    if y > 0 && matrix[x][y - 1] == 0 {
                        y -= 1;
                    } else {
                        direction = Direction::Up;
                        x -= 1;
                    }
                }
                Direction::Up => {
                    if x > 0 && matrix[x - 1][y] == 0 {
                        x -= 1;
                    } else {
                        direction = Direction::Right;
                        y += 1;
                    }
                }
            }
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_matrix_1() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn test_generate_matrix_2() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }

    #[test]
    fn test_generate_matrix_3() {
        assert_eq!(
            Solution::generate_matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );
    }

    #[test]
    fn test_generate_matrix_4() {
        assert_eq!(
            Solution::generate_matrix(5),
            vec![
                vec![1, 2, 3, 4, 5],
                vec![16, 17, 18, 19, 6],
                vec![15, 24, 25, 20, 7],
                vec![14, 23, 22, 21, 8],
                vec![13, 12, 11, 10, 9]
            ]
        );
    }

    #[test]
    fn test_generate_matrix_5() {
        assert_eq!(
            Solution::generate_matrix(6),
            vec![
                vec![1, 2, 3, 4, 5, 6],
                vec![20, 21, 22, 23, 24, 7],
                vec![19, 32, 33, 34, 25, 8],
                vec![18, 31, 36, 35, 26, 9],
                vec![17, 30, 29, 28, 27, 10],
                vec![16, 15, 14, 13, 12, 11]
            ]
        );
    }

    #[test]
    fn test_generate_matrix_6() {
        assert_eq!(
            Solution::generate_matrix(7),
            vec![
                vec![1, 2, 3, 4, 5, 6, 7],
                vec![24, 25, 26, 27, 28, 29, 8],
                vec![23, 40, 41, 42, 43, 30, 9],
                vec![22, 39, 48, 49, 44, 31, 10],
                vec![21, 38, 47, 46, 45, 32, 11],
                vec![20, 37, 36, 35, 34, 33, 12],
                vec![19, 18, 17, 16, 15, 14, 13]
            ]
        );
    }
}
