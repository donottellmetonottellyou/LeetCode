pub struct Solution;

impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        //  We filter out the zeros and check to see if there are any non-zero elements, or else we
        //  can only create a strength of 0.
        let non_zero = nums
            .iter()
            .filter_map(|num| if *num == 0 { None } else { Some(*num as i64) });
        let mut safety_check = non_zero.clone();
        match safety_check.next() {
            Some(num) if num.is_negative() && safety_check.next().is_none() => {
                //  Also make sure we aren't given only one negative number.
                if nums.contains(&0) {
                    //  If we were also given zeros, we can return zero.
                    return 0;
                } else {
                    //  If we are given only one negative number, we have to return the negative
                    //  number.
                    return num;
                }
            }
            Some(_) => {}
            None => return 0,
        }

        //  Here we keep track of how many negatives there are and which is the minimum negative (in
        //  magnitude). If we have an odd number of negatives, we have to divide out the smallest to
        //  get the maximum strength.
        let mut minimum_negative = None;
        let mut product: i64 = non_zero
            .map(|num| {
                if num.is_negative() {
                    if let Some(negative) = minimum_negative.as_mut() {
                        if *negative < num {
                            *negative = num;
                        }
                    } else {
                        minimum_negative = Some(num);
                    }
                }

                num
            })
            .product();
        if product.is_negative() {
            product /= minimum_negative.unwrap();
        }

        product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_maximum_strength_1() {
        let nums = vec![3, -1, -5, 2, 5, -9];

        assert_eq!(Solution::max_strength(nums), 1350);
    }

    #[test]
    fn finds_maximum_strength_2() {
        let nums = vec![-4, -5, -4];

        assert_eq!(Solution::max_strength(nums), 20);
    }

    #[test]
    fn finds_maximum_strength_3() {
        let nums = vec![0, -1];

        assert_eq!(Solution::max_strength(nums), 0);
    }
}
