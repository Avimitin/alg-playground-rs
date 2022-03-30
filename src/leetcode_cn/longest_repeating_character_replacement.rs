#[allow(dead_code)]
struct Solution;

impl Solution {
    fn get(arr: &Vec<i32>, c: char) -> i32 {
        arr[c as usize - 'A' as usize]
    }

    fn set(arr: &mut Vec<i32>, c: char, i: i32) {
        arr[c as usize - 'A' as usize] = i;
    }

    #[allow(dead_code)]
    pub fn character_replacement(s: String, k: i32) -> i32 {
        if s.len() < 2 {
            return 1;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut alpha_met = vec![0; 26];

        let mut pat = chars[0];
        let mut i = 0;
        let mut j = i + 1;

        if chars[j] != pat {}
        // let now = Self::get(&alpha_met, chars[ptr]);
        // Self::set(&mut alpha_met, chars[ptr], now + 1);

        unimplemented!()
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::character_replacement("ABAB".to_string(), 2));
    assert_eq!(4, Solution::character_replacement("AABABBA".to_string(), 1));
    assert_eq!(4, Solution::character_replacement("ABBB".to_string(), 2));
    assert_eq!(7, Solution::character_replacement("KRSCDCSONAJNHLBMDQGIFCPEKPOHQIHLTDIQGEKLRLCQNBOHNDQGHJPNDQPERNFSSSRDEQLFPCCCARFMDLHADJADAGNNSBNCJQOF".to_string(), 4))
}
