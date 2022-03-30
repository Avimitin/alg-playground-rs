#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut rec = Vec::new();

        for (i, row) in mat.iter().enumerate() {
            let sum: i32 = row.iter().sum();
            rec.push(vec![i as i32, sum]);
        }

        rec.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut ans = Vec::new();
        for r in rec.iter().take(k as usize) {
            ans.push(r[0])
        }
        ans
    }
}

#[test]
fn test() {}
