struct Solution;

impl Solution {
    pub fn longest_palindrome(m: String) -> String {
        let s = m.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let (mut start, mut len) = (0,1);

        for i in 0..n { dp[i][i] = true; }
        for i in 0..n-1 {
            if s[i] == s[i+1] {
                dp[i][i+1] = true;
                if len < 2 { len = 2; start = i; }
            }
        }
        for l in 3..=n {
            for i in 0..n-l+1 {
                let j = i+l-1;

                if dp[i+1][j-1] && s[i] == s[j] {
                    dp[i][j] = true;

                    if l > len {
                        len = l;
                        start = i;
                    }
                }
            }
        }
        m[start..start+len].to_string()
    }
}

fn main() {
    println!("{}", Solution::longest_palindrome("adjkhkahd".to_string()));
}
