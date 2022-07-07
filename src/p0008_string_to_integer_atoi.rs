pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn my_atoi(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        
        let mut chars = s.chars().peekable();

        while let Some(' ') = chars.peek() {
            chars.next();
        }
        
        let positive = match chars.peek() {
            Some('+') => {
                chars.next();
                true
            },
            Some('-') => {
                chars.next();
                false
            },
            Some(_) => true,
            None => return 0,
        };

        while let Some('0') = chars.peek() {
            chars.next();
        }

        let mut digits = vec![];

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' => {
                    chars.next();
                    digits.push(c);
                },
                _ => break,
            }
        }

        if digits.is_empty() {
            return 0;
        }

        let mut x = 0_i32;
        let mut p = match 10_i32.checked_pow(digits.len() as u32 - 1) {
            Some(n) => n,
            None => return if positive { i32::MAX } else { i32::MIN },
        };

        for d in digits {
            let y = match p.checked_mul(d.to_digit(10).unwrap() as i32) {
                Some(n) => n,
                None => return if positive { i32::MAX } else { i32::MIN },
            };
            x = match x.checked_add(y) {
                Some(n) => n,
                None => return if positive { i32::MAX } else { i32::MIN },
            };
            p /= 10;
        }

        if positive {
            x
        } else {
            x.checked_mul(-1).unwrap_or(i32::MIN)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(42, Solution::my_atoi("42".to_owned()));
        assert_eq!(-42, Solution::my_atoi("   -42".to_owned()));
        assert_eq!(4193, Solution::my_atoi("4193 with words".to_owned()));
        assert_eq!(i32::MIN, Solution::my_atoi("-82147483648".to_owned()));
        assert_eq!(i32::MAX, Solution::my_atoi("82147483648".to_owned()));
        assert_eq!(0, Solution::my_atoi("words and 987".to_owned()));
        assert_eq!(0, Solution::my_atoi("+-12".to_owned()));
        assert_eq!(0, Solution::my_atoi("-+12".to_owned()));
        assert_eq!(0, Solution::my_atoi("- 12".to_owned()));
        assert_eq!(12345678, Solution::my_atoi("  0000000000012345678".to_owned()));
        assert_eq!(0, Solution::my_atoi("00000-42a1234".to_owned()));
        assert_eq!(3, Solution::my_atoi("3.14159".to_owned()));
        assert_eq!(-12, Solution::my_atoi("  -0012a42".to_owned()));
    }
}