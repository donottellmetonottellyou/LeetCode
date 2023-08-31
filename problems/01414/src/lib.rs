pub struct Solution;

#[derive(Default)]
struct Fibonacci {
    last: i32,
    current: i32,
}
impl Fibonacci {
    fn next(&mut self) -> Option<i32> {
        let next = self.last.checked_add(self.current)?;
        self.last = self.current;
        self.current = next;

        self.current = self.current.max(1);

        Some(self.current)
    }

    fn previous(&mut self) -> Option<i32> {
        if self.last == 0 {
            return None;
        }

        let previous = self.current - self.last;
        self.current = self.last;
        self.last = previous;

        Some(self.current)
    }
}

impl Solution {
    pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
        let mut fibonacci = Fibonacci::default();
        while let Some(next) = fibonacci.next() {
            if k - next < 0 {
                break;
            }
        }

        let mut n = 0;

        while let Some(fib_number) = fibonacci.previous() {
            while k >= 0 {
                k -= fib_number;
                n += 1;
            }
            k += fib_number;
            n -= 1;

            if k == 0 {
                break;
            }
        }

        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for k in (1..=101).step_by(10) {
            dbg!(Solution::find_min_fibonacci_numbers(k));
        }
    }
}
