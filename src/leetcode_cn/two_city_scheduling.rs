#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_by(|a, b| (b[1] - b[0]).cmp(&(a[1] - a[0])));
        let mut sums = 0;
        for (i, pair) in costs.iter().enumerate() {
            if i < costs.len() / 2 {
                sums += pair[0];
            } else {
                sums += pair[1];
            }
        }
        sums
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::two_city_sched_cost(vec![
            vec![10, 20],
            vec![30, 200],
            vec![400, 50],
            vec![30, 20]
        ]),
        110
    );
    assert_eq!(
        Solution::two_city_sched_cost(vec![
            vec![259, 770],
            vec![448, 54],
            vec![926, 667],
            vec![184, 139],
            vec![840, 118],
            vec![577, 469]
        ]),
        1859
    );
    assert_eq!(
        Solution::two_city_sched_cost(vec![
            vec![515, 563],
            vec![451, 713],
            vec![537, 709],
            vec![343, 819],
            vec![855, 779],
            vec![457, 60],
            vec![650, 359],
            vec![631, 42]
        ]),
        3086
    );
}
