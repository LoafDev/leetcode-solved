struct Solution;

impl Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let (mut i, mut j) = (0, n-1);
        let mut res = 0;

        while i < j {
            if height[i] < height[j] {
                res = std::cmp::max(res, height[i] * (j-i) as i32);
                i += 1;
            } else {
                res = std::cmp::max(res, height[j] * (j-i) as i32);
                j -= 1;
            }
        }

        res
    }
}

fn main() {
    assert_eq!(49, Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
    assert_eq!(1, Solution::max_area(vec![1,1]));
}
