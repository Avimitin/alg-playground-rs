#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 1 {
            return vec![vec![1]];
        }
        let (mut r1, mut r2) = (0_usize, (n - 1) as usize);
        let (mut c1, mut c2) = (0_usize, (n - 1) as usize);
        let n = n as usize;

        let mut ans = vec![vec![0; n]; n];
        let mut cnt = 0;
        while r2 >= r1 && c2 >= c1 {
            for i in c1..=c2 {
                cnt += 1;
                ans[r1][i] = cnt;
            }

            // This is more readable
            #[allow(clippy::needless_range_loop)]
            for i in r1 + 1..=r2 {
                cnt += 1;
                ans[i][c2] = cnt;
            }

            for i in (c1..=c2 - 1).rev() {
                cnt += 1;
                ans[r2][i] = cnt;
            }

            for i in (r1 + 1..=r2 - 1).rev() {
                cnt += 1;
                ans[i][c1] = cnt;
            }

            r1 += 1;
            r2 -= 1;
            c1 += 1;
            c2 -= 1;
        }

        ans
    }
}

#[test]
fn test() {}
