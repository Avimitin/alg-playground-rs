use std::collections::hash_set::HashSet;

struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (a, b, c) = (s1.len(), s2.len(), s3.len());
        // unsafe index!
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();

        if a + b != c {
            return false;
        }

        let mut stack = vec![(0, 0)];
        let mut visited = HashSet::new();

        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();

            if x + y == c {
                return true;
            }

            if x + 1 <= a && s1[x] == s3[x + y] && !visited.contains(&(x + 1, y)) {
                stack.push((x + 1, y));
                visited.insert((x + 1, y));
            }

            if y + 1 <= b && s2[y] == s3[x + y] && !visited.contains(&(x, y + 1)) {
                stack.push((x, y + 1));
                visited.insert((x, y + 1));
            }
        }

        false
    }
}

#[test]
fn test() {}
