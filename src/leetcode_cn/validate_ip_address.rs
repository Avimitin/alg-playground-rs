#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn valid_ip_address(query_ip: String) -> String {
        let mut part: Vec<&str> = query_ip.split('.').collect();
        if part.len() == 4 {
            return Self::handle_ipv4(&part);
        } else {
            part = query_ip.split(":").collect();
        }

        if part.len() == 8 {
            return Self::handle_ipv6(&part);
        } else {
            return String::from("Neither");
        }
    }

    fn handle_ipv4(part: &Vec<&str>) -> String {
        for pa in part {
            if pa.len() < 1 || pa.len() > 3 {
                return String::from("Neither");
            }
            let chars: Vec<char> = pa.chars().collect();
            if pa.len() > 1 && chars[0] == '0' {
                return String::from("Neither");
            }
            let val = match pa.parse::<i32>() {
                Ok(v) => v,
                Err(_) => return String::from("Neither"),
            };
            if val < 0 || val > 255 {
                return String::from("Neither");
            }
        }

        String::from("IPv4")
    }

    fn handle_ipv6(part: &Vec<&str>) -> String {
        for pa in part {
            if pa.len() < 1 || pa.len() > 4 {
                return String::from("Neither");
            }

            let chars: Vec<char> = pa.chars().collect();
            for c in chars {
                match c.to_digit(16) {
                    Some(_) => (),
                    None => return String::from("Neither"),
                }
            }
        }

        String::from("IPv6")
    }
}

#[test]
fn test() {
    let result = Solution::valid_ip_address("172.16.254.1".to_string());
    assert_eq!(result, "IPv4");

    let result = Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string());
    assert_eq!(result, "IPv6");

    let result = Solution::valid_ip_address("256.256.256.256".to_string());
    assert_eq!(result, "Neither");

    let result = Solution::valid_ip_address("01.168.1.1".to_string());
    assert_eq!(result, "Neither");

    let result = Solution::valid_ip_address("12..33.4".to_string());
    assert_eq!(result, "Neither");

    let result = Solution::valid_ip_address("127.0.0.1".to_string());
    assert_eq!(result, "IPv4");
}
