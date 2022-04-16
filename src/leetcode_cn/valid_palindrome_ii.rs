#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn valid_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        let (mut i, mut j) = (0, chars.len() - 1);
        while i <= j {
            if chars[i] != chars[j] {
                return Self::is_palindrome(&chars, i + 1, j)
                    || Self::is_palindrome(&chars, i, j - 1);
            }

            i += 1;
            j -= 1;
        }

        true
    }

    fn is_palindrome(cs: &[char], mut lo: usize, mut hi: usize) -> bool {
        while lo < hi {
            if cs[lo] != cs[hi] {
                return false;
            }
            lo += 1;
            hi -= 1;
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::valid_palindrome("abca".to_string()));
    assert!(Solution::valid_palindrome("aba".to_string()));
    assert!(Solution::valid_palindrome("abc".to_string()));
    assert!(Solution::valid_palindrome("deeee".to_string()));
}
