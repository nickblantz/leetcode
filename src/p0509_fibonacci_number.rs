struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fib(n: i32) -> i32 {
        const SQRT_5: f32 = 2.23606797749978969640917366873127623544061835961152572427089;
        const PHI: f32 = (1.0 + SQRT_5) / 2.0;
        const PSI: f32 = 1.0 - PHI;
        const X: f32 = 1.0 / SQRT_5;
        
        ((X * PHI.powi(n)) - (X * PSI.powi(n))) as i32
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(0, Solution::fib(0));
        assert_eq!(1, Solution::fib(1));
        assert_eq!(1, Solution::fib(2));
        assert_eq!(2, Solution::fib(3));
        assert_eq!(3, Solution::fib(4));
        assert_eq!(5, Solution::fib(5));
        assert_eq!(8, Solution::fib(6));
        assert_eq!(13, Solution::fib(7));
        assert_eq!(21, Solution::fib(8));
        assert_eq!(34, Solution::fib(9));
        assert_eq!(55, Solution::fib(10));
    }
}