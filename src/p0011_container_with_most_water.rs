struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area2(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut max = 0;
        
        for i in 0..(len - 1) {
            for j in (i + 1)..len {
                let next = (j - i) as i32 * height[i].min(height[j]);
                
                if next > max {
                    max = next;
                }
            }
        }

        max
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut head = 0;
        let mut tail = len - 1;
        let mut max = 0;

        while head < tail {
            let next = (tail - head) as i32 * height[head].min(height[tail]);
                
            if next > max {
                max = next;
            }

            if height[head] < height[tail] {
                head += 1;
            } else {
                tail -= 1;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(49, Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
        assert_eq!(1, Solution::max_area(vec![1,1]));
        assert_eq!(100, Solution::max_area(vec![100,100]));
        assert_eq!(100, Solution::max_area(vec![100,100]));
        assert_eq!(300, Solution::max_area(vec![100,200,200,100]));
    }
}
