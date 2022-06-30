#[allow(dead_code)]
pub fn min_operations(nums: Vec<i32>, goal: i32) -> i32 {
    let length = nums.len();
    let target_sum = nums.iter().sum::<i32>() - goal;
    let mut longest_span: i32 = -1;
    let mut current_sum = 0;
    let mut left_index = 0;

    for right_index in 0..length {
        current_sum += nums[right_index];

        while current_sum > target_sum && left_index <= right_index {
            current_sum -= nums[left_index];
            left_index += 1;
        }

        if current_sum == target_sum {
            longest_span = longest_span.max(right_index as i32 - left_index as i32 + 1);
        }
    }

    if longest_span == -1 {
        return longest_span;
    }

    length as i32 - longest_span
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(2, min_operations(vec![3, 2, 4, 1, 1], 5));
        assert_eq!(2, min_operations(vec![1, 1, 4, 2, 3], 5));
        assert_eq!(-1, min_operations(vec![5, 6, 7, 8, 9], 4));
        assert_eq!(5, min_operations(vec![3, 2, 20, 1, 1, 3], 10));
        assert_eq!(-1, min_operations(vec![1, 1], 3));
        assert_eq!(4, min_operations(vec![5, 2, 2, 1, 2, 2, 4], 10));
        assert_eq!(5, min_operations(vec![5, 2, 1, 1, 2, 2, 4], 10));
        assert_eq!(
            16,
            min_operations(
                vec![
                    8828, 9581, 49, 9818, 9974, 9869, 9991, 10000, 10000, 10000, 9999, 9993, 9904,
                    8819, 1231, 6309,
                ],
                134365,
            )
        );
        assert_eq!(
            113,
            min_operations(
                vec![
                    5207, 5594, 477, 6938, 8010, 7606, 2356, 6349, 3970, 751, 5997, 6114, 9903,
                    3859, 6900, 7722, 2378, 1996, 8902, 228, 4461, 90, 7321, 7893, 4879, 9987,
                    1146, 8177, 1073, 7254, 5088, 402, 4266, 6443, 3084, 1403, 5357, 2565, 3470,
                    3639, 9468, 8932, 3119, 5839, 8008, 2712, 2735, 825, 4236, 3703, 2711, 530,
                    9630, 1521, 2174, 5027, 4833, 3483, 445, 8300, 3194, 8784, 279, 3097, 1491,
                    9864, 4992, 6164, 2043, 5364, 9192, 9649, 9944, 7230, 7224, 585, 3722, 5628,
                    4833, 8379, 3967, 5649, 2554, 5828, 4331, 3547, 7847, 5433, 3394, 4968, 9983,
                    3540, 9224, 6216, 9665, 8070, 31, 3555, 4198, 2626, 9553, 9724, 4503, 1951,
                    9980, 3975, 6025, 8928, 2952, 911, 3674, 6620, 3745, 6548, 4985, 5206, 5777,
                    1908, 6029, 2322, 2626, 2188, 5639
                ],
                565610,
            )
        );
    }
}
