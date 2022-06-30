#[allow(dead_code)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (smaller, mut larger) = if nums1.len() < nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };

    larger.reserve(smaller.len());
    for element in smaller {
        let index = larger.partition_point(|&x| x < element);
        larger.insert(index, element);
    }

    let len = larger.len();
    if len % 2 == 0 {
        (larger[len / 2 - 1] + larger[len / 2]) as f64 / 2.0
    } else {
        larger[len / 2] as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(2.0, find_median_sorted_arrays(vec![1, 3], vec![2]));
        assert_eq!(2.5, find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
        assert_eq!(
            3.0,
            find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5])
        );
        assert_eq!(
            4.0,
            find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 6, 7])
        );
        assert_eq!(
            2.0,
            find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![0, 1, 2])
        );
        assert_eq!(
            3.0,
            find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![2, 3, 4])
        );
        assert_eq!(
            4.0,
            find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![4, 5, 6])
        );
    }
}
