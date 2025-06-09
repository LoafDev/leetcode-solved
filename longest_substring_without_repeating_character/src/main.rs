struct Solution;

impl Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() { return 0; }

        let a = s.as_bytes();
        let mut v = vec![false; 128];
        let mut maxn = 0;
        let mut l=0;

        for r in 0..a.len() {
            while v[a[r] as usize] {
                v[a[l] as usize] = false;
                l += 1;
            }
            v[a[r] as usize] = true;
            maxn = std::cmp::max(maxn,r-l+1);
        }
        maxn as i32
    }
}

fn main() {
    assert_eq!(6, Solution::length_of_longest_substring(String::from("ashdqw")));
}
