use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.iter().map(|i| (*i, 0)).collect::<HashMap<i32, u8>>();
        let mut counter = 0;

        for n in &nums {
            if set.get(&(*n - 1)).is_some() {
                continue;
            }

            let mut current_count = 1;

            let mut cur = *n;
            while set.get(&(cur + 1)).is_some() {
                cur += 1;
                current_count += 1;
            }

            counter = std::cmp::max(counter, current_count);
        }

        counter
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
}
