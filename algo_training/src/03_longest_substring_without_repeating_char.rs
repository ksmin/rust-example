use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_set = HashSet::new();
        let mut max_len = 0;
        let s_len = s.len();
        for idx in 0..s_len {
            char_set.clear();
            let sub_str = String::from(&s[idx..]);
            let mut sub_max_len = 0;
            for (sub_idx, ch) in sub_str.chars().enumerate() {
                if char_set.contains(&ch) {
                    break;
                }
                sub_max_len += 1;
                char_set.insert(ch);
            }
            if max_len < sub_max_len {
                max_len = sub_max_len;
            }
        }
        if max_len == 0 {
            max_len = s_len;
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
    }
}

fn main() {
}