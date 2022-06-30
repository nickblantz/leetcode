struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let front = k - 1;
        let back = card_points.len() - 1;
        let mut current_sum = card_points[0..=front].iter().sum();
        let mut largest_sum = current_sum;

        for i in 0..k {
            current_sum += card_points[back - i] - card_points[front - i];

            if current_sum > largest_sum {
                largest_sum = current_sum;
            }
        }

        largest_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score() {
        assert_eq!(12, Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3));
        assert_eq!(4, Solution::max_score(vec![2, 2, 2], 2));
        assert_eq!(55, Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7));
    }
}
