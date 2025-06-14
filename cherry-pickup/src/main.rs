struct Solution;

impl Solution {
    fn recur(x1: usize, y1: usize, x2: usize, grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        let y2 = x1 + y1 - x2;
        let n = grid.len();

        if x1 >= n || y1 >= n || x2 >= n || y2 >= n || grid[x1][y1] == -1 || grid[x2][y2] == -1 { return -1; }
        else if x1 == n - 1 && y1 == n - 1 && x2 == n - 1 && y2 == n - 1 { return grid[n-1][n-1]; }
        else if dp[x1][y1][x2] != -2 { return dp[x1][y1][x2]; }

        let val = if x1 != x2 || y1 != y2 { grid[x1][y1] + grid[x2][y2] } else { grid[x1][y1] };
        let (
            down_down,
            down_right,
            right_down,
            right_right
        ) = (
            Self::recur(x1+1, y1, x2+1, grid, dp),
            Self::recur(x1+1, y1, x2, grid, dp),
            Self::recur(x1, y1+1, x2+1, grid, dp),
            Self::recur(x1, y1+1, x2, grid, dp)
        );
        let tp = [down_down, down_right, right_down, right_right].into_iter().max().unwrap();

        dp[x1][y1][x2] = if tp != -1 { val + tp } else { -1 };
        dp[x1][y1][x2]
    }

    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 1 { return if grid[0][0] != -1 { grid[0][0] } else { 0 }; }

        let mut dp = vec![
            vec![
                vec![-2; grid.len()]; grid.len()
            ]; grid.len()
        ];

        let res = Self::recur(0,0,0, &grid, &mut dp);
        std::cmp::max(res,0)
    }
}

fn main() {
    assert_eq!(5, Solution::cherry_pickup(
        vec![
            vec![0,1,-1],
            vec![1,0,-1],
            vec![1,1,1]
        ]
    ));
}
