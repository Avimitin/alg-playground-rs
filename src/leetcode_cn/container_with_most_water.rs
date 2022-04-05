#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{max, min};

        let cal_area = |x: usize, y: usize| -> usize { x * y };

        let mut max_area = 0;
        let (mut i, mut j) = (0, height.len() - 1);

        while i < j {
            let x = j - i;
            let y = min(height[i], height[j]);
            max_area = max(max_area, cal_area(x, y as usize));

            if height[i] > height[j] {
                j -= 1;
            } else {
                i += 1;
            }
        }

        max_area as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
