#[allow(dead_code)]
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let (_min, min_i, sum) = gas.iter().zip(cost).enumerate().fold(
        (i32::max_value(), 0, 0),
        |(min, min_i, sum), (i, (g, c))| {
            let next = sum + (g - c);

            if next < min {
                (next, i, next)
            } else {
                (min, min_i, next)
            }
        },
    );

    if sum >= 0 {
        ((min_i + 1) % gas.len()) as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(
            3,
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
        );
        assert_eq!(-1, can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]));
        assert_eq!(0, can_complete_circuit(vec![3, 1, 1], vec![1, 2, 2]));
    }
}
