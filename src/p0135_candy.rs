use std::cmp::Ordering;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn candy_fast(ratings: Vec<i32>) -> i32 {
        fn count(n: i32) -> i32 {
            (n * (n + 1)) / 2
        }
        
        let len = ratings.len();
    
        if len < 2 {
            return len as i32;
        }

        let mut candies = 0;
        let mut up = 0;
        let mut down = 0;
        let mut old_slope = 0;
        
        for i in 1..len {
            let new_slope = if ratings[i] > ratings[i - 1] { 1 } 
                else if ratings[i] < ratings[i - 1] { -1 } 
                else { 0 };

            if (old_slope > 0 && new_slope == 0) || (old_slope < 0 && new_slope >= 0) {
                candies += count(up) + count(down) + up.max(down);
                up = 0;
                down = 0;
            }

            if new_slope > 0 {
                up += 1;
            } else if new_slope < 0 {
                down += 1;
            } else {
                candies += 1;
            }

            old_slope = new_slope;
        }
        
        candies += count(up) + count(down) + up.max(down) + 1;
        candies
    }

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
