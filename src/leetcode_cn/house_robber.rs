#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;

        if nums.is_empty() {
            return 0;
        }

        if nums.len() < 2 {
            return nums[0];
        }

        if nums.len() < 3 {
            return max(nums[0], nums[1]);
        }

        if nums.len() < 4 {
            return max(nums[1], nums[0] + nums[2]);
        }

        let mut a = nums[0];
        let mut b = max(a, nums[1]);

        for i in 2..nums.len() {
            let c = max(b, a + nums[i]);
            a = b;
            b = c;
        }

        max(a, b)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}
