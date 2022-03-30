#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn simplify_path(path: String) -> String {
        if path.is_empty() {
            return "".to_string();
        }

        let mut ans = Vec::new();

        for part in path.split('/') {
            match part {
                "." | "" => continue,
                ".." => {
                    ans.pop();
                }
                // normal component just push it into result
                _ => {
                    ans.push(part);
                },
            }
        }

        format!("/{}", ans.join("/"))
    }
}

#[test]
fn test() {
    assert_eq!("/home/foo", Solution::simplify_path("/home//foo".to_string()));
    assert_eq!("/", Solution::simplify_path("/../".to_string()));
}
