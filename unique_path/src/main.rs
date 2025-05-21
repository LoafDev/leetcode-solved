struct Solution;

impl Solution {
    pub fn recur(m: usize, n: usize, dp: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if i >= m || j >= n { return 0; }
        else if dp[i][j] != -1 { return dp[i][j]; }
        else if i == m - 1 && j == n - 1 { return 1; }

        let (
            right,
            down
        ) = (
            Self::recur(m, n, dp, i, j+1),
            Self::recur(m, n, dp, i+1, j)
        );

        dp[i][j] = right + down;
        dp[i][j]
    }

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![-1; n as usize]; m as usize];
        Self::recur(m as usize , n as usize, &mut dp, 0, 0)
    }
}

fn main() {
    println!("{}", Solution::unique_paths(3,2));
}

