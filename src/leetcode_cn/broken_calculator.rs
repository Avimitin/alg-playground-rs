#[allow(dead_code)]
struct Solution;

// By reversing the operation i.e instead of multiplying we go for dividing and
// instead of subtarcting we go for adding.
// There would be also possibly three cases in this...
// 1. If target == startValue then it will be zero.
// 2. If  target > startValue then we only divide it by 2 till we get startValue ==
// target or startValue get greater than target.
//
// IN STEP SECOND WE HAVE ONE MORE SUB CASES.
// We only go for division if target is even else we only increase target by 1.
// 3. If target < startValue then we only increment it by 1.
//
// Example
// I/P  startValue = 5   target = 8
// Step - 1 target > startValue then divide target by 2 , it becomes 4.
// Step - 2 target < startValue then we only add by 1 till we get target ==
// startValue. We can only find it by subtracting startValue - target value as
// startValue is > than target in this case.
// Therfore total no of steps taken is divide->add i.e '2' which is the answer.
impl Solution {
    #[allow(dead_code)]
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        // we need them to be multible
        if start_value == target {
            return 0;
        }
        let mut t = target;
        let mut counter = 0;
        while start_value < t {
            counter += 1;

            if t % 2 == 0 {
                t /= 2;
            } else {
                t += 1;
            }
        }

        counter += start_value - t;

        counter
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::broken_calc(2, 3));
    assert_eq!(2, Solution::broken_calc(5, 8));
    assert_eq!(3, Solution::broken_calc(3, 10));
}
