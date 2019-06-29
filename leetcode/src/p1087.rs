struct Solution;

impl Solution {
    fn exp(s: &[u8], ans: Vec<String>) -> Vec<String> {
        if s.len() == 0 {
            return ans;
        }
        let mut idx = 1;
        let mut letters = Vec::new();

        if s[0] == b'{' {
            while s[idx] != b'}' {
                let ch = s[idx] as char;
                if ch.is_ascii_alphanumeric() {
                    letters.push(ch);
                }
                idx += 1;
            }
            letters.sort();
            idx += 1;
        } else {
            letters.push(s[0] as char);
        }

        let new_ans = ans
            .iter()
            .map(|s| {
                letters
                    .iter()
                    .map(|ch| format!("{}{}", s, ch))
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>();

        Solution::exp(&s[idx..], new_ans)
    }

    pub fn expand(s: String) -> Vec<String> {
        Solution::exp(s.as_bytes(), vec!["".to_string()])
    }
}

fn main() {
    println!("{:?}", Solution::expand("{b,a}c{d,e}f".to_string()));
}
