struct Solution;

impl Solution {
    fn recur(i: usize, j: usize, dungeon: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
        let (m,n) = (dungeon.len(), dungeon[0].len());

        if i >= m || j >= n { return i32::MAX; }
        else if i == m - 1 && j == n - 1 { return if dungeon[i][j] <= 0 { 1 - dungeon[i][j] } else { 1 }; }
        else if dp[i][j] != -1 { return dp[i][j]; }

        let (
            up,
            left
        ) = (
            Self::recur(i+1,j, dungeon, dp),
            Self::recur(i,j+1, dungeon, dp)
        );

        dp[i][j] = std::cmp::min(up, left) - dungeon[i][j];
        if dp[i][j] <= 0 { dp[i][j] =  1; }
        dp[i][j]
    }

    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        Self::recur(0, 0, &dungeon, &mut vec![
            vec![-1; dungeon[0].len()];
            dungeon.len()
        ])
    }
}

fn main() {
    assert_eq!(Solution::calculate_minimum_hp(vec![
        vec![-2,-3,3],
        vec![-5,-10,1],
        vec![10,30,-5]
    ]), 7);
    assert_eq!(Solution::calculate_minimum_hp(vec![vec![0]]), 1);
}
