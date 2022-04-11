#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        if grid.is_empty() {
            return grid;
        }

        let (r, c) = (grid.len(), grid[0].len());
        let mut ans = vec![vec![0; c]; r];
        let k = k as usize;

        for (i, row) in grid.iter().enumerate() {
            for (j, num) in row.iter().enumerate().take(grid[i].len()) {
                let next_row = (i + ((j + k) / c)) % r;
                let next_col = (j + k) % c;
                ans[next_row][next_col] = *num;
            }
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
        vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8],]
    );
}
