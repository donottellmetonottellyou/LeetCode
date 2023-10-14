pub struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    /// Finds the result of propogating asteroid collisions.
    ///
    /// Assumes:
    /// 2 <= asteroids.len() <= 1_000
    /// -1_000 <= asteroids[i] <= 1_000
    /// asteroids[i] != 0
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        for asteroid in asteroids {
            loop {
                if asteroid.is_positive() {
                    stack.push(asteroid);
                    break;
                }

                let last_asteroid = if let Some(last_asteroid) = stack.last().copied() {
                    if last_asteroid.is_negative() {
                        stack.push(asteroid);
                        break;
                    } else {
                        last_asteroid
                    }
                } else {
                    stack.push(asteroid);
                    break;
                };

                match last_asteroid.abs().cmp(&asteroid.abs()) {
                    Less => {
                        stack.pop();
                    }
                    Equal => {
                        stack.pop();
                        break;
                    }
                    Greater => break,
                }
            }
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collides_asteroids_1() {
        let asteroids = vec![5, 10, -5];
        let solution = vec![5, 10];

        assert_eq!(Solution::asteroid_collision(asteroids), solution);
    }

    #[test]
    fn collides_asteroids_2() {
        let asteroids = vec![8, -8];
        let solution = vec![];

        assert_eq!(Solution::asteroid_collision(asteroids), solution);
    }

    #[test]
    fn collides_asteroids_3() {
        let asteroids = vec![10, 2, -5];
        let solution = vec![10];

        assert_eq!(Solution::asteroid_collision(asteroids), solution);
    }

    #[test]
    fn collides_asteroids_4() {
        let asteroids = vec![-2, 1, -2, -2];
        let solution = vec![-2, -2, -2];

        assert_eq!(Solution::asteroid_collision(asteroids), solution);
    }

    #[test]
    fn collides_asteroids_5() {
        let asteroids = vec![-2, 1, -2, -1];
        let solution = vec![-2, -2, -1];

        assert_eq!(Solution::asteroid_collision(asteroids), solution);
    }

    #[test]
    fn collides_asteroids_6() {
        let asteroids = vec![-2, 1, -1, -2];
        let solution = vec![-2, -2];

        assert_eq!(Solution::asteroid_collision(asteroids), solution);
    }
}
