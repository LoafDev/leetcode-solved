struct Solution;

const MOD: i32 = 1_000_000_000 + 7;
impl Solution {
    fn quick_prime(n: i32) -> bool { n == 2 || n == 3 || n == 5 || n == 7 }

    pub fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
        let n = s.len();
        let mut dp = vec![0; n];

        for i in 0..n {
            dp[i] = std::cmp::max(dp[i], dp[i-j] + 1);
        }
       
        dp[n-1]
    }
}

fn main() {
    assert_eq!(Solution::beautiful_partitions("23542185131".to_string(), 3,2), 3);
    assert_eq!(Solution::beautiful_partitions("23542185131".to_string(), 3,3), 1);
    assert_eq!(Solution::beautiful_partitions("3312958".to_string(), 3,1), 1);
}
