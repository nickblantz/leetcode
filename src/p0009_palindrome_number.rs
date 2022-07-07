pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn is_palindrome_fast(x: i32) -> bool {
        let mut n1 = x;
        let mut n2: i32 = 0;

        while n1 > 0 {
            n2 = n2 * 10 + n1 % 10;
            n1 = n1 / 10;
        }

        x == n2
    }

    fn divisor(n: u32) -> u32 {
        const POWERS: [u32; 10] = [
            0, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
        ];
        const MAX_DIGITS: [u32; 33] = [
            1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9,
            9, 10, 10, 10,
        ];

        let bits = 32 - n.leading_zeros();
        let digits = MAX_DIGITS[bits as usize];

        if n < POWERS[digits as usize - 1] {
            return POWERS[digits as usize - 2];
        }

        POWERS[digits as usize - 1]
    }

    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        let x = x as u32;
        let mut high = Self::divisor(x);
        let mut low = 1;

        while high >= low {
            if (x / high) % 10 != (x / low) % 10 {
                return false;
            }

            high /= 10;
            low *= 10;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(false, Solution::is_palindrome(-1000));
        assert_eq!(true, Solution::is_palindrome(0));
        assert_eq!(true, Solution::is_palindrome(1));
        assert_eq!(true, Solution::is_palindrome(11));
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(true, Solution::is_palindrome(1221));
        assert_eq!(true, Solution::is_palindrome(12321));
        assert_eq!(true, Solution::is_palindrome(123321));
        assert_eq!(false, Solution::is_palindrome(123));
        assert_eq!(false, Solution::is_palindrome(4536));
        assert_eq!(false, Solution::is_palindrome(10000));
        assert_eq!(false, Solution::is_palindrome(23475));
        assert_eq!(false, Solution::is_palindrome(834765));
    }
}
