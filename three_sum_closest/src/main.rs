struct Solution;

impl Solution {
    fn binary_search(nums: &Vec<i32>, n: usize, start: usize, target: i32) -> Option<usize> {
        let (
            mut left,
            mut right,
            mut c,
            mut d
        ) = (
            start+1,
            n-1,
            None,
            None
        );

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target { return Some(mid); }
            else if nums[mid] < target { c = Some(mid); left = mid + 1; }
            else { d = Some(mid); right = mid - 1; }
        }
        if let None = c { d }
        else if let None = d { c }
        else if let None = c { None }
        else {
            if (target - nums[c.unwrap()]).abs() < (target - nums[d.unwrap()]).abs() { c }
            else { d }
        }
    }

    fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut best_record = i32::MAX;
        let mut best_sum = 0;

        for i in 0..n-2 {
            for j in i+1..n-1 {

                let sum = nums[i] + nums[j];
                let tp;

                if let Some(idx) = Self::binary_search(&nums, n, j, target - sum) { tp = idx; }
                else { continue; }

                let tempo = (target - (sum + nums[tp])).abs();

                if best_record > tempo { best_record = tempo; best_sum = sum + nums[tp]; }
                if best_record == 0 { return target; }
            }
        }

        best_sum
    }
}

fn main() {
    assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    assert_eq!(0, Solution::three_sum_closest(vec![0; 3], 1));
}
