pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let negative = x < 0;
        let mut x = match x.checked_abs() {
            Some(n) => n,
            None => return 0,
        };
        let mut y: i32 = 0;
        let len = (x as f32).log10() as u32;
        let mut p = 10_i32.pow(len);
        
        for _ in 0..=len {
            let next = match (x % 10).checked_mul(p) {
                Some(n) => n,
                None => return 0,
            };
            y = match y.checked_add(next) {
                Some(n) => n,
                None => return 0,
            };
            x /= 10;
            p /= 10;
        }

        if negative {
            y.checked_mul(-1).unwrap_or(0)
        } else {
            y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(-321, Solution::reverse(-123));
        assert_eq!(21, Solution::reverse(120));
        assert_eq!(-21, Solution::reverse(-1200000));
        assert_eq!(0, Solution::reverse(1123456789));
        assert_eq!(0, Solution::reverse(-2147483648));
    }
}