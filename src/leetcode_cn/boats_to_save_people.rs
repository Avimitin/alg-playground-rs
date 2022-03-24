#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    #[cold]
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        if people.is_empty() {
            return 0;
        }

        let mut people = people.clone();
        people.sort_unstable();

        let (mut i, mut j, mut ans) = (people.len() - 1, 0, 0);
        while i >= j {
            ans += 1;
            if people[i] + people[j] <= limit {
                j += 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
    assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
    assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    assert_eq!(Solution::num_rescue_boats(vec![3, 1, 7], 7), 2);
}
