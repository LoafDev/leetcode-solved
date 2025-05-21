struct Solution;

impl Solution {
    fn recur_path(grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<(i32, Vec<(usize, usize)>)>>, mut path: Vec<(usize, usize)>, i: usize, j: usize) -> (i32, Vec<(usize, usize)>) {
        if i >= grid.len() || j >= grid.len() { return (i32::MIN, path); }
        else if grid[i][j] == -1 { return (i32::MIN, path); }
        else if dp[i][j].0 != -1 { return dp[i][j].clone(); }
        else if i == grid.len() - 1 && j == grid.len() - 1 { return (grid[i][j], path); }

        let (mut right, mut down) = ((i32::MIN, Vec::new()), (i32::MIN, Vec::new()));

        if j+1 < grid.len() && grid[i][j+1] != -1 {
            path.push((i, j+1));
            right = Self::recur_path(grid, dp, path.clone(), i, j+1);
            path.pop();
        }

        if i+1 < grid.len() && grid[i+1][j] != -1 {
            path.push((i+1, j));
            down = Self::recur_path(grid, dp, path.clone(), i+1, j);
            path.pop();
        }

        if right > down { dp[i][j] = (grid[i][j] + right.0, right.1); dp[i][j].clone() }
        else { dp[i][j] = (grid[i][j] + down.0, down.1); dp[i][j].clone() }
    }

    fn recur_home(grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, i: isize, j: isize) -> i32 {
        if i < 0 || j < 0 { return i32::MIN; }
        else if grid[i as usize][j as usize] == -1 { return i32::MIN; }
        else if dp[i as usize][j as usize] != -1 { return dp[i as usize][j as usize]; }
        else if i == 0 || j == 0 { return grid[i as usize][j as usize]; }

        let (left, up) =
        (
            Self::recur_home(&grid, dp, i, j-1),
            Self::recur_home(&grid, dp, i-1, j)
        );

        dp[i as usize][j as usize] = grid[i as usize][j as usize] + std::cmp::max(left, up);
        dp[i as usize][j as usize]
    }

    pub fn cherry_pickup(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 1 { return grid[0][0]; }

        let mut dp = vec![
            vec![(-1, Vec::new()); grid.len()];
            grid.len()
        ];
        let mut dp_home = vec![
            vec![-1; grid.len()];
            grid.len()
        ];

        let a = Self::recur_path(&grid, &mut dp, Vec::new(), 0,0);
        for i in &a.1 { grid[i.0][i.1] = 0; }
        println!("{:?}", grid);
        let b = Self::recur_home(&grid, &mut dp_home, (grid.len()-1) as isize , (grid.len()-1) as isize);

        if a.0 + b >= 0 { a.0 + b } 
        else { 0 }
    }
}

fn main() {
    println!("{}", Solution::cherry_pickup(
        vec![
            vec![0,0,0],
            vec![-1,-1,0],
            vec![0,1,1]
        ]
    ));
}
