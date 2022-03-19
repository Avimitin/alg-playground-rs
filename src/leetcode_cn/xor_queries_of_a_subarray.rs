#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut t = 0;
        let xores = arr
            .iter()
            .map(|x| {
                t ^= x;
                t
            })
            .collect::<Vec<i32>>();

        queries
            .iter()
            .map(|rng| {
                if rng[0] > 0 {
                    xores[rng[0] as usize - 1] ^ xores[rng[1] as usize]
                } else {
                    xores[rng[1] as usize]
                }
            })
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::xor_queries(
            vec![1, 3, 4, 8],
            vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
        ),
        vec![2, 7, 14, 8]
    );
}
