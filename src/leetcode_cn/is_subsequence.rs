#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let mut j = 0;
        let mut sp = s.chars();
        let mut sc = sp.next().unwrap();
        for c in t.chars() {
            if sc == c {
                j += 1;
                if j == s.len() {
                    return true;
                }
                sc = sp.next().unwrap();
            }
        }

        false
    }
}

#[test]
fn try_sol() {
    // should pass
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    assert!(Solution::is_subsequence(s, t));

    // shouldn't pass
    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    assert!(!Solution::is_subsequence(s, t));

    let s = "acb".to_string();
    let t = "ahbgdc".to_string();
    assert!(!Solution::is_subsequence(s, t));
}
