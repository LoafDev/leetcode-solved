struct Solution;

impl Solution {
    pub fn count_substrings(d: String) -> i32 {
        let s = d.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut res = 0;

        for i in 0..n { dp[i][i] = true; res += 1; }
        for i in 0..n-1 { if s[i] == s[i+1] { dp[i][i+1] = true; res += 1; } }

        for length in 3..=n {
            for i in 0..n-length+1 {
                let j = i + length - 1;
                if dp[i+1][j-1] && s[i] == s[j] { dp[i][j] = true; res += 1; }
            }
        }

        res
    }
}

fn main() {
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    assert_eq!(Solution::count_substrings("fdsklf".to_string()), 6);
}
