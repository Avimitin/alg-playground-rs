#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        if nums.len() < 2 {
            return nums[0];
        }

        let mut dp = vec![0; 10001];
        for n in nums {
            dp[n as usize] += n;
        }

        let mut a = dp[0];
        let mut b = std::cmp::max(a, dp[1]);

        for i in 2..10001 {
            let c = std::cmp::max(b, a + dp[i]);
            a = b;
            b = c;
        }

        std::cmp::max(a, b)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}
