struct Solution;

impl Solution {
    pub fn recur(grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if i >= grid.len() || j >= grid[0].len() { return i32::MAX; }
        else if dp[i][j] != -1 { return dp[i][j]; }
        else if i == grid.len() - 1 && j == grid[0].len() - 1 { return grid[i][j]; }

        let (
            right,
            down
        ) = (
            Self::recur(grid, dp, i, j+1),
            Self::recur(grid, dp, i+1, j)
        );

        dp[i][j] = grid[i][j] + std::cmp::min(right, down);
        dp[i][j]
    }

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![-1; grid[0].len()]; grid.len()];
        Self::recur(&grid, &mut dp, 0, 0)
    }
}

fn main() {
    println!("{}", Solution::min_path_sum(vec![
        vec![1,3,1],
        vec![1,5,1],
        vec![4,2,1]
    ]));
}
