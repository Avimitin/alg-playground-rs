#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut hi = 0;
        let mut lo = 0;
        for n in &nums {
            hi += n;
            lo = std::cmp::max(lo, *n);
        }

        let mut possible_value = 0;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if Self::count_split(&nums, mid) <= m {
                hi = mid - 1;
                possible_value = mid;
            } else {
                lo = mid + 1;
            }
        }

        possible_value
    }

    fn count_split(nums: &[i32], max: i32) -> i32 {
        let mut sum = 0;
        let mut splits = 0;
        for n in nums {
            sum += n;
            if sum > max {
                sum = *n;
                splits += 1;
            }
        }

        splits + 1
    }
}

#[test]
fn test() {}
