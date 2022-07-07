struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive_fast(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        
        let num_set: HashSet<_> = nums.into_iter().collect();
        let mut ans = 0;

        for &num in &num_set {
            if !num_set.contains(&(num - 1)) {
                let count = (num..).take_while(|x| num_set.contains(x)).count();
                ans = ans.max(count);
            }
        }

        ans as i32
    }
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use core::ops::RangeInclusive;

        let len = nums.len();

        if len == 0 {
            return 0;
        }
        
        let mut max = 1;
        let mut seen: HashMap<i32, RangeInclusive<i32>> = HashMap::with_capacity(len);

        for num in nums {
            if seen.contains_key(&num) {
                continue;
            }

            let mut updates = vec![];

            let start = match seen.get(&(num - 1)) {
                Some(r1) => {
                    let start = r1.start();
                    updates.push(*start);
                    *start
                },
                None => num,
            };

            let end = match seen.get(&(num + 1)) {
                Some(r1) => {
                    let end = r1.end();
                    updates.push(*end);
                    *end
                },
                None => num,
            };

            let range = start..=end;
            let count = range.size_hint().0;

            if count > max {
                max = count;
            }

            seen.insert(num, range);
            for update in updates {
                seen.insert(update, start..=end);
            }
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(7, Solution::longest_consecutive(vec![-10, -9, -8, -7, -6, -5, 7, 4, 1, 6, 2, 5, 3]));
        assert_eq!(7, Solution::longest_consecutive(vec![7, 4, 1, 6, 2, 5, 3]));
        assert_eq!(4, Solution::longest_consecutive(vec![-10, -9, -8, 100,4,200,1,3,2]));
        assert_eq!(4, Solution::longest_consecutive(vec![100,4,200,1,3,2]));
        assert_eq!(18, Solution::longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1,10,22,12,32,32,10,11, 12,13,14,15,16,17,18,19,20,-10,-9,-8,-7,-6,-5,-4,-3,-2,21,22,23,24,25,26,27]));
    }
}

// 5 
//   4 -> 1
//   6 -> 1

// 3
//   2 -> 1
//   4 -> 1

// 1
//   0 -> 1
//   4 -> 2