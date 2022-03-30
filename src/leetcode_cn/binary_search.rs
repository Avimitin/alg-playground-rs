#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;

        while lo <= hi {
            let mid = (lo + hi) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        -1
    }
}

#[test]
fn test() {}
