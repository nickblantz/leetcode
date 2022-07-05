use std::cmp::Ordering;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let len = ratings.len();
    
        if len < 2 {
            return 1;
        }
        
        let mut maximas = std::collections::HashMap::new();
        let mut sum = len as i32;

        let mut inc = 1;
        let mut windows = ratings.windows(2).enumerate();
        let mut last_maxima = None;
        
        while let Some((i, &[a, b])) = windows.next() {
            let i = i + 1;

            match b.cmp(&a) {
                Ordering::Greater => {
                    sum += inc;
                    last_maxima = Some((i, inc));
                    inc += 1;
                },
                _ => {
                    if let Some(maxima) = last_maxima {
                        maximas.insert(maxima.0, maxima.1);
                        last_maxima = None;
                    }
                    inc = 1;
                },
            }
        }
        
        let mut inc = 1;
        let mut windows = ratings.windows(2).rev().enumerate();
        
        while let Some((i, &[a, b])) = windows.next() {
            let i = len - 2 - i;

            match a.cmp(&b) {
                Ordering::Greater => {
                    if let Some(&maxima) = maximas.get(&(i)) {
                        if maxima < inc {
                            sum -= maxima;
                            sum += inc;
                            inc += 1;
                        }
                    } else {
                        sum += inc;
                        inc += 1;
                    }
                },
                _ => {
                    inc = 1;
                },
            }
        }

        sum
    }

    pub fn candy3(ratings: Vec<i32>) -> i32 {
        let len = ratings.len();

        if len < 2 {
            return 1;
        }

        let mut windows = ratings.windows(2).enumerate();
        let mut sum = 0;
        let mut incr = 1;
        let mut is_incr = ratings[0] < ratings[1];

        while let Some((_, &[a, b])) = windows.next() {
            println!("{}, {} | {} + {} = {}", a, b, sum, incr, sum + incr);
            println!("is incr: {}", is_incr);
            sum += incr;

            if is_incr {
                if a < b {
                    incr += 1;
                } else {
                    is_incr = !is_incr;
                    incr = 1;
                }
            } else {
                if a > b {
                    incr += 1;
                } else {
                    is_incr = !is_incr;
                    incr = 1;
                }
            }
        }
        
        println!("{} + {} = {}", sum, incr, sum + incr);
        println!("is incr: {}", is_incr);
        sum += incr;

        sum
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candy() {
        assert_eq!(5, Solution::candy(vec![1, 0, 2]));
        assert_eq!(4, Solution::candy(vec![1, 2, 2]));
        assert_eq!(6, Solution::candy(vec![1, 2, 3]));
        assert_eq!(7, Solution::candy(vec![1, 2, 3, 1]));
        assert_eq!(6, Solution::candy(vec![3, 2, 1]));
        assert_eq!(9, Solution::candy(vec![1, 2, 3, 2, 1]));
        assert_eq!(13, Solution::candy(vec![1, 2, 3, 2, 1, 0]));
        assert_eq!(13, Solution::candy(vec![0, 1, 2, 3, 2, 1]));
        assert_eq!(4, Solution::candy(vec![1, 3, 2]));
        assert_eq!(5, Solution::candy(vec![1, 3, 2, 2]));
        assert_eq!(5, Solution::candy(vec![2, 2, 3, 1]));
        assert_eq!(22, Solution::candy(vec![1, 0, 0, 4, 5, 5, 4, 7, 4, 3, 1]));
        assert_eq!(22, Solution::candy(vec![1, 3, 4, 7, 4, 5, 5, 4, 0, 0, 1]));
    }
}


// 1 3 2 2
// 1 2 1 1