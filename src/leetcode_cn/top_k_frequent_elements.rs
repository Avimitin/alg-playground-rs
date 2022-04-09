#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn top_k_frequent(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;

        if nums.len() <= k as usize {
            return nums;
        }

        nums.sort_unstable();
        let mut counter = BinaryHeap::new();
        let mut freq = 1;
        for i in 1..nums.len() {
            // [0, len - 1)
            if nums[i] == nums[i - 1] {
                freq += 1;
            } else {
                counter.push((freq, nums[i - 1]));
                freq = 1;
            }
        }

        counter.push((freq, *nums.last().unwrap()));

        let mut ans = Vec::new();
        for _ in 0..k {
            ans.push(counter.pop().unwrap().1)
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
        vec![1, 2]
    );
    assert_eq!(
        Solution::top_k_frequent(vec![-1, -1], 1),
        vec![-1]
    );
    assert_eq!(
        Solution::top_k_frequent(vec![1,1,1,2,2,2,3,3,3], 3),
        vec![1, 2, 3]
    );
}
