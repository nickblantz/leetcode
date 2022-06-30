#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut diffs = std::collections::HashMap::with_capacity(nums.len());
    nums.iter().enumerate().for_each(|(i, x)| {
        diffs.insert(target - x, i);
    });
    nums.iter()
        .enumerate()
        .find_map(|(i, x)| match diffs.get(x) {
            Some(&j) if i != j => Some(vec![i as i32, j as i32]),
            _ => None,
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6));
    }
}
