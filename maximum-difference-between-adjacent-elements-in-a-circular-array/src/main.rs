struct Solution;

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut maxn = i32::MIN;

        for i in 0..n {
            maxn = std::cmp::max(maxn, (nums[i] - nums[(i+1) % n]).abs());
        }

        maxn
    }
}
fn main() {
    assert_eq!(3, Solution::max_adjacent_distance(vec![1,2,4]));
    assert_eq!(5, Solution::max_adjacent_distance(vec![-5,-10,-5]));
}
