struct Solution;

impl Solution {
    fn recur(i: isize, j: isize, mut health: i32, dungeon: &Vec<Vec<i32>>) -> i32 {
        if i < 0 || j < 0 || (health + dungeon[i as usize][j as usize] <= 0 && i != (dungeon.len() - 1) as isize && j != (dungeon[0].len() - 1) as isize) { return i32::MAX; }
        else if i == 0 && j == 0 { return health.max(1); }

        if i != (dungeon.len() - 1) as isize && j != (dungeon[0].len() - 1) as isize { health += dungeon[i as usize][j as usize]; }
        let (
            up,
            left
        ) = (
            Self::recur(i-1,j, health, dungeon),
            Self::recur(i,j-1, health, dungeon)
        );

        std::cmp::min(up, left)
    }

    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m,n) = (dungeon.len()-1, dungeon[0].len()-1);
        Self::recur(m as isize, n as isize, 1, &dungeon)
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
