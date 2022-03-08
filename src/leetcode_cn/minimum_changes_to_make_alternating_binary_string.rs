#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_operation(s: String) -> i32 {
        if s.len() < 2 {
            return 0;
        }

        let chars = s.chars().collect::<Vec<char>>();
        let mut ans_a: i32 = 0;
        let mut ans_b: i32 = 0;

        let mut i = 0;
        while i < chars.len() - 1 {
            // 01010101
            ans_a += Self::cmp_add(chars[i], '0');
            ans_a += Self::cmp_add(chars[i + 1], '1');

            // 10101010
            ans_b += Self::cmp_add(chars[i], '1');
            ans_b += Self::cmp_add(chars[i + 1], '0');

            i += 2;
        }

        if {chars.len() % 2} != 0 {
            ans_a += Self::cmp_add(chars[chars.len()-1], '0');
            ans_b += Self::cmp_add(chars[chars.len()-1], '1');
        }

        std::cmp::min(ans_a, ans_b)
    }

    fn cmp_add(left: char, right: char) -> i32 {
        if left == right {
            0
        } else {
            1
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_operation("0101".to_string()), 0);
    assert_eq!(Solution::min_operation("0111".to_string()), 1);
    assert_eq!(Solution::min_operation("010000".to_string()), 2);
    assert_eq!(Solution::min_operation("101010".to_string()), 0);
    assert_eq!(Solution::min_operation("111111".to_string()), 3);
}
