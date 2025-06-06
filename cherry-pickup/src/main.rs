struct Solution;

impl Solution {
    fn recur(x1: usize, y1: usize, x2: usize, is_end: &mut bool, grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        let y2 = x1 + y1 - x2;
        let n = grid.len();

        if x1 >= n || y1 >= n || x2 >= n || y2 >= n { return -1; }
        else if grid[x1][y1] == -1 || grid[x2][y2] == -1 { return -1; }
        else if x1 == n - 1 && y1 == n - 1 && x2 == n - 1 && y2 == n - 1 { *is_end = true; return grid[n-1][n-1]; }
        else if dp[x1][y1][x2] != -1 { return dp[x1][y1][x2]; }

        let val = if x1 != x2 || y1 != y2 { grid[x1][y1] + grid[x2][y2] } else { grid[x1][y1] };
        let (
            down_down,
            down_right,
            right_down,
            right_right
        ) = (
            Self::recur(x1+1, y1, x2+1, is_end, grid, dp),
            Self::recur(x1+1, y1, x2, is_end, grid, dp),
            Self::recur(x1, y1+1, x2+1, is_end, grid, dp),
            Self::recur(x1, y1+1, x2, is_end, grid, dp)
        );
        let tp = [down_down, down_right, right_down, right_right].into_iter().max().unwrap();

        if tp != -1 { dp[x1][y1][x2] = val + tp; }
        dp[x1][y1][x2]
    }

    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 1 { return if grid[0][0] != -1 { grid[0][0] } else { 0 }; }

        let mut is_end = false;
        let mut dp = vec![
            vec![
                vec![-1; grid.len()]; grid.len()
            ]; grid.len()
        ];

        let res = Self::recur(0,0,0, &mut is_end, &grid, &mut dp);
        if is_end { res } else { 0 }
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
