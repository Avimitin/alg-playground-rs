#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_splits(s: String) -> i32 {
        // n = s.length
        let n = s.len();

        // this vector contains counter for not individual chars
        let mut left: Vec<u32> = vec![0; n + 2];
        let mut right: Vec<u32> = vec![0; n + 2];

        // this vector contains 26 letters.
        let mut lb: Vec<bool> = vec![false; 26];
        let mut rb: Vec<bool> = vec![false; 26];

        let mut i: usize = 1;
        for c in s.chars() {
            let c = c as usize - 'a' as usize;

            // if this character `c` is already shown
            if lb[c] {
                // store the individual chars number
                left[i] = left[i - 1];
            } else {
                // this character is shown *now*
                lb[c] = true;
                // So we increase the individual char counter
                left[i] = left[i - 1] + 1;
            }

            i += 1;
        }

        // this is as the same as the above, just reverse
        let mut i: usize = n;
        for c in s.chars().rev() {
            let c = c as usize - 'a' as usize;

            if rb[c] {
                right[i] = right[i + 1];
            } else {
                rb[c] = true;
                right[i] = right[i + 1] + 1;
            }

            i -= 1;
        }

        let mut ans = 0;
        for i in 1..n {
            if left[i] == right[i+1] {
                ans += 1;
            }
        }

        ans
    }
}

#[test]
fn test_solution() {
    let s = "aacaba".to_string();
    assert_eq!(Solution::num_splits(s), 2);

    let s = "abcd".to_string();
    assert_eq!(Solution::num_splits(s), 1);

    let s = "aaaaa".to_string();
    assert_eq!(Solution::num_splits(s), 4);

    let s = "acbadbaada".to_string();
    assert_eq!(Solution::num_splits(s), 2);
}
