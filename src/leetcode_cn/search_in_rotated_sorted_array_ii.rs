#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut privot = 0;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                privot = i - 1;
                break;
            }
        }

        if Self::binary_search(&nums, target, 0, privot + 1) {
            return true;
        }

        Self::binary_search(&nums, target, privot + 1, nums.len())
    }

    fn binary_search(v: &Vec<i32>, target: i32, mut lo: usize, mut hi: usize) -> bool {
        while lo < hi {
            let mid = (lo + hi) / 2;
            if v[mid] == target {
                return true;
            } else if v[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        false
    }
}

#[test]
fn test() {
    assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    assert!(Solution::search(vec![2, 2, 2, 3, 2, 2, 2], 3));
}
