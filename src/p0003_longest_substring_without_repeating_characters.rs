#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let length = s.len();
    let mut maximum = 0;
    let mut cur_sum = 0;
    let mut win_tail = 0;
    let mut window = std::collections::HashSet::with_capacity(length);

    for win_head in 0..length {
        let next = s[win_head];

        if window.contains(&next) {
            if cur_sum > maximum {
                maximum = cur_sum;
            }

            for i in win_tail..win_head {
                cur_sum -= 1;
                win_tail += 1;
                window.remove(&s[i]);

                if s[i] == next {
                    break;
                }
            }
        }

        cur_sum += 1;
        window.insert(next);
    }

    maximum.max(cur_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(3, length_of_longest_substring("abcabcbb".into()));
        assert_eq!(1, length_of_longest_substring("bbbbb".into()));
        assert_eq!(3, length_of_longest_substring("pwwkew".into()));
    }
}
