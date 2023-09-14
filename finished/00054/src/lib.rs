pub struct Solution;

use std::collections::VecDeque;

/// Simple direction iterator to keep track of what to do.
#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
impl Iterator for Direction {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let current = *self;

        match self {
            Self::Right => *self = Self::Down,
            Self::Down => *self = Self::Left,
            Self::Left => *self = Self::Up,
            Self::Up => *self = Self::Right,
        }

        Some(current)
    }
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        //  Make the matrix into VecDeque's so we can access pop_front().
        let mut matrix: VecDeque<_> = matrix.into_iter().map(VecDeque::from).collect();
        let mut spiral = Vec::with_capacity(matrix.len() * matrix[0].len());

        for direction in Direction::Right.into_iter() {
            match direction {
                Direction::Right => match matrix.pop_front() {
                    Some(top) => spiral.append(&mut top.into()),
                    None => break,
                },
                Direction::Down => {
                    for row in &mut matrix {
                        match row.pop_back() {
                            Some(rightmost) => spiral.push(rightmost),
                            None => break,
                        }
                    }
                }
                Direction::Left => match matrix.pop_back() {
                    Some(bottom) => spiral.append(&mut bottom.into_iter().rev().collect()),
                    None => break,
                },
                Direction::Up => {
                    for row in matrix.iter_mut().rev() {
                        match row.pop_front() {
                            Some(leftmost) => spiral.push(leftmost),
                            None => break,
                        }
                    }
                }
            }
        }

        spiral
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spirals_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let solution = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];

        assert_eq!(Solution::spiral_order(matrix), solution);
    }

    #[test]
    fn spirals_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let solution = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];

        assert_eq!(Solution::spiral_order(matrix), solution);
    }
}
