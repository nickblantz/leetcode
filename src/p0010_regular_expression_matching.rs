struct Solution {}

#[allow(dead_code)]
impl Solution {
    fn dfs(
        si: usize,
        pi: usize,
        ss: &Vec<char>,
        ps: &Vec<char>,
        memo: &mut Vec<Vec<Option<bool>>>,
    ) -> bool {
        // if we reach the end of the pattern,
        // return true if the input is exhausted
        if pi == ps.len() {
            return si == ss.len();
        }

        // if we've already checked this input position
        // for this pattern position, return the memo
        if let Some(memo) = memo[si][pi] {
            return memo;
        }

        // we haven't run out of input and our current input character matches
        let first_matches = si < ss.len() && (ps[pi] == '.' || ps[pi] == ss[si]);

        // if the next pattern is a repetition
        let result = if pi + 1 < ps.len() && ps[pi + 1] == '*' {
            // result is if the current character matches and the remaining input is
            // matched (check next input against current pattern)
            // or skip the repetition pattern (check current input against next pattern)
            first_matches && Self::dfs(si + 1, pi, ss, ps, memo)
                || Self::dfs(si, pi + 2, ss, ps, memo)
        } else {
            // result is if the current character matches and the remaining input is
            // matched (check next input against next pattern)
            first_matches && Self::dfs(si + 1, pi + 1, ss, ps, memo)
        };

        // memoize the result
        memo[si][pi] = Some(result);

        result
    }

    pub fn is_match(s: String, p: String) -> bool {
        Self::dfs(
            0,
            0,
            &s.chars().collect(),
            &p.chars().collect(),
            &mut (0..(s.len() + 1))
                .map(|_| {
                    (0..(p.len() + 1))
                        .map(|_| None)
                        .collect::<Vec<Option<bool>>>()
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(true, Solution::is_match("aa".into(), "aa".into()));
        assert_eq!(false, Solution::is_match("a".into(), "aa".into()));
        assert_eq!(false, Solution::is_match("aaa".into(), "aa".into()));
        assert_eq!(true, Solution::is_match("aa".into(), "..".into()));
        assert_eq!(false, Solution::is_match("a".into(), "..".into()));
        assert_eq!(false, Solution::is_match("aaa".into(), "..".into()));
        assert_eq!(true, Solution::is_match("aaa".into(), "a*".into()));
        assert_eq!(true, Solution::is_match("aaa".into(), "a*a".into()));
        assert_eq!(true, Solution::is_match("aaa".into(), "a*aa".into()));
        assert_eq!(true, Solution::is_match("aaa".into(), "a*.a".into()));
        assert_eq!(true, Solution::is_match("aaa".into(), "a*a.".into()));
        assert_eq!(false, Solution::is_match("aa".into(), "a".into()));
        assert_eq!(true, Solution::is_match("ab".into(), ".*".into()));
        assert_eq!(true, Solution::is_match("abaaba".into(), "aba.*aba".into()));
        assert_eq!(true, Solution::is_match("abaa".into(), "aba.*.".into()));
        assert_eq!(true, Solution::is_match("abaaba".into(), "abaa*aba".into()));
        assert_eq!(true, Solution::is_match("abaa".into(), "abaa*.".into()));
        assert_eq!(true, Solution::is_match("aba".into(), ".*aba.*".into()));
        assert_eq!(
            true,
            Solution::is_match("aaaaaabaaaaa".into(), "a*abaa*".into())
        );
        assert_eq!(true, Solution::is_match("aab".into(), "aa*abz*".into()));
        assert_eq!(true, Solution::is_match("aabzzzz".into(), "aa*abz*".into()));
        assert_eq!(true, Solution::is_match("ab".into(), "a.*b.*".into()));
        assert_eq!(false, Solution::is_match("aa".into(), "a.*b.*".into()));
        assert_eq!(
            false,
            Solution::is_match("mississippi".into(), "mis*is*p*.".into())
        );
        assert_eq!(
            true,
            Solution::is_match("mississippi".into(), "mis*is*ip*.".into())
        );
        assert_eq!(true, Solution::is_match("aab".into(), "c*a*b".into()));
    }
}
