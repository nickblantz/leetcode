#[allow(dead_code)]
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let len = cost.len();
    let mut costs = cost.iter().enumerate();
    let mut dp: Vec<i32> = (0..len).map(|_| 0).collect();

    dp[0] = *costs.next().expect("cost too short").1;

    match costs.next() {
        Some((_, c)) => dp[1] = *c,
        None => return dp[0],
    }

    costs.for_each(|(i, c)| dp[i] = dp[i - 1].min(dp[i - 2]) + c);

    dp[len - 1].min(dp[len - 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(15, min_cost_climbing_stairs(vec![10, 15, 20]));
        assert_eq!(
            6,
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }
}
