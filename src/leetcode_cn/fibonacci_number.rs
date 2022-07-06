struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }

        let (mut a, mut b) = (0, 1);

        for _ in 2..=n {
            let sum = a + b;
            a = b;
            b = sum;
        }

        b
    }
}

#[test]
fn test() {
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(3), 2);
    assert_eq!(Solution::fib(4), 3);
}
