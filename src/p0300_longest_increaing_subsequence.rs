#[allow(dead_code)]
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    *nums
        .iter()
        .enumerate()
        .fold(
            (0..(nums.len())).map(|_| 0).collect::<Vec<i32>>(),
            |mut acc, (i, x)| {
                acc[i] = 1 + nums[0..i]
                    .iter()
                    .enumerate()
                    .filter(|(_, y)| x > y)
                    .map(|(j, _)| acc[j])
                    .max()
                    .unwrap_or(0);
                acc
            },
        )
        .iter()
        .max()
        .unwrap_or(&0)
}
