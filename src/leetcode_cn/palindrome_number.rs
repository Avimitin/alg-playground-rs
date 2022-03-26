#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        let mut v = Vec::new();
        let mut n = x;
        while n > 0 {
            let i = n % 10;
            v.push(i);
            n /= 10;
        }
        let mut i = 0;
        v.reverse();
        let sum = v.iter().fold(0, |acc, x| {
            let sum = acc + x * 10_i32.pow(i);
            i+=1;
            sum
        });

        x == sum
    }
}

#[test]
fn test() {
    assert!(Solution::is_palindrome(131));
    assert!(!Solution::is_palindrome(123));
    assert!(Solution::is_palindrome(14541));
}
