#[allow(dead_code)]
pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    let mut maximum = 0;
    let mut cur_sum = 0;
    let mut win_tail = 0;
    let mut window = std::collections::HashSet::with_capacity(length);

    for win_head in 0..length {
        let next = nums[win_head];

        if window.contains(&next) {
            if cur_sum > maximum {
                maximum = cur_sum;
            }

            for i in win_tail..win_head {
                cur_sum -= nums[i];
                win_tail += 1;
                window.remove(&nums[i]);

                if nums[i] == next {
                    break;
                }
            }
        }

        cur_sum += next;
        window.insert(next);
    }

    maximum.max(cur_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_unique_subarray() {
        assert_eq!(15, maximum_unique_subarray(vec![1, 2, 3, 4, 5]));
        assert_eq!(9, maximum_unique_subarray(vec![2, 3, 4]));
        assert_eq!(4, maximum_unique_subarray(vec![3, 1, 1]));
        assert_eq!(17, maximum_unique_subarray(vec![4, 2, 4, 5, 6]));
        assert_eq!(8, maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]));
    }
}
