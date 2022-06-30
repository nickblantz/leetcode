struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::with_capacity(0);
        }

        let s = s.bytes().collect::<Vec<u8>>();
        let mut longest_span = (0, 0);

        for i in 0..(s.len()) {
            longest_span = Self::check_palindrome(&s, i + 1, i + 1, longest_span);
            longest_span = Self::check_palindrome(&s, i + 1, i + 2, longest_span);
        }

        String::from_utf8(
            s[(longest_span.0)..=(longest_span.1)]
                .iter()
                .map(|c| c.clone())
                .collect(),
        )
        .expect("Invalid UTF8")
    }

    fn check_palindrome(
        s: &Vec<u8>,
        start: usize,
        stop: usize,
        longest_span: (usize, usize),
    ) -> (usize, usize) {
        let len = s.len();
        let mut stop = stop;
        let mut start = start;
        let mut pred = stop <= len && start > 0 && s[stop - 1] == s[start - 1];

        if !pred {
            return longest_span;
        }

        while pred {
            stop += 1;
            start -= 1;
            pred = stop <= len && start > 0 && s[stop - 1] == s[start - 1];
        }

        if (stop - 2) - start > longest_span.1 - longest_span.0 {
            (start, stop - 2)
        } else {
            longest_span
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(
            String::from("ccc"),
            Solution::longest_palindrome("ccc".into())
        );
        assert_eq!(
            String::from("aba"),
            Solution::longest_palindrome("aba".into())
        );
        assert_eq!(
            String::from("bab"),
            Solution::longest_palindrome("babad".into())
        );
        assert_eq!(
            String::from("bb"),
            Solution::longest_palindrome("cbbd".into())
        );
        assert_eq!(
            String::from("baab"),
            Solution::longest_palindrome("cbaabd".into())
        );
        assert_eq!(
            String::from("abbbbba"),
            Solution::longest_palindrome("xabbbbbay".into())
        );
        assert_eq!(
            String::from("abbbba"),
            Solution::longest_palindrome("xabbbbay".into())
        );
        assert_eq!(
            String::from("bbbb"),
            Solution::longest_palindrome("abbbb".into())
        );
        assert_eq!(
            String::from("bbbb"),
            Solution::longest_palindrome("bbbba".into())
        );
    }
}
