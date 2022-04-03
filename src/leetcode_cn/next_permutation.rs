#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut j = 0;
        let mut largest = true;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                largest = false;
                j = i;
                break;
            }
        }

        if largest {
            nums.sort_unstable();
            return;
        }

        let mut k = 0;
        // the numbers range is [0, 100]
        let mut min = 101;
        for i in j + 1..nums.len() {
            if nums[i] > nums[j] && nums[i] <= min {
                min = nums[i];
                k = i;
            }
        }

        nums.swap(j, k);

        let mut left = j + 1;
        let mut right = nums.len() - 1;
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

#[test]
fn test() {
    let mut v = vec![1, 2, 3];
    Solution::next_permutation(&mut v);
    assert_eq!(v, vec![1, 3, 2]);

    let mut v = vec![1, 3, 2];
    Solution::next_permutation(&mut v);
    assert_eq!(v, vec![2, 1, 3]);

    let mut v = vec![1, 1, 5];
    Solution::next_permutation(&mut v);
    assert_eq!(v, vec![1, 5, 1]);

    let mut v = vec![2, 3, 1, 3, 3];
    Solution::next_permutation(&mut v);
    assert_eq!(v, vec![2, 3, 3, 1, 3]);
}
