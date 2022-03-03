#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_time(time: String) -> String {
        // get vector with value ['?', '?', ':', '?', '?']
        let mut chars: Vec<char> = time.chars().collect();

        let mut output = String::new();

        // if time start with ?, test the second char
        if chars[0] == '?' {
            // if time start with ??, it must be 24
            if chars[1] == '?' {
                chars[0] = '2';
                chars[1] = '3';
            } else {
                // if start with ?x, x = 4,5,6,7,8,9, then ? must be 1
                if chars[1] as usize > '3' as usize {
                    chars[0] = '1';
                } else {
                    chars[0] = '2';
                }
            }
        }

        // ?? is handled, so no need to handle it again
        if chars[1] == '?' {
            if chars[0] == '1' || chars[0] == '0' {
                chars[1] = '9';
            } else {
                chars[1] = '3';
            }
        }

        // emit the :

        // if start with ??
        if chars[3] == '?' {
            chars[3] = '5';
        }

        if chars[4] == '?' {
            chars[4] = '9';
        }

        for c in chars {
            output.push(c);
        }

        output
    }
}
