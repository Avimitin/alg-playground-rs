#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    #[cold]
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut records = Vec::new();
        let mut sum = 0;
        for op in ops {
            match op.as_str() {
                "C" => {
                    let last = records.pop().unwrap();
                    sum -= last;
                }
                "D" => {
                    let cal = 2 * records.last().unwrap();
                    records.push(cal);
                    sum += cal;
                }
                "+" => {
                    let cal = records.last().unwrap() + records[records.len() - 2];
                    records.push(cal);
                    sum += cal;
                }
                _ => {
                    let par = op.parse::<i32>().unwrap();
                    records.push(par);
                    sum += par;
                }
            }
        }

        sum
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::cal_points(vec![
            "5".to_string(),
            "2".to_string(),
            "C".to_string(),
            "D".to_string(),
            "+".to_string()
        ]),
        30
    );
    assert_eq!(
        Solution::cal_points(vec![
            "5".to_string(),
            "-2".to_string(),
            "4".to_string(),
            "C".to_string(),
            "D".to_string(),
            "9".to_string(),
            "+".to_string(),
            "+".to_string()
        ]),
        27
    );
}
