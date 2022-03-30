#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut s = nums[0];
        let mut f = nums[nums[0] as usize];
        while s != f {
            s = nums[s as usize];
            f = nums[nums[f as usize] as usize];
        }

        let mut s = 0;
        while s != f {
            s = nums[s as usize];
            f = nums[f as usize];
        }

        s
    }
}

#[test]
fn test() {}
