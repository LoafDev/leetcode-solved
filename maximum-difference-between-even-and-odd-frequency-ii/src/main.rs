struct Solution;

impl Solution {
    fn num_status(a: usize, b: usize) -> usize {
        ((a & 1) << 1) | (b & 1)
    }

    pub fn max_difference(s: String, k: i32) -> i32 {
        let mut ans = i32::MIN;

        for a in '0'..='4' {
            for b in '0'..='4' {
                if a == b { continue; }
                let mut best = [i32::MAX; 4];

                let (mut ca, mut cb) = (0,0);
                let (mut pa, mut pb) = (0,0);
                let mut left = -1;

                for (r, c) in s.chars().enumerate().map(|(r,c)| (r as i32, c)) {
                    ca += (c == a) as usize;
                    cb += (c == b) as usize;

                    while r - left >= k && cb - pb >= 2 {
                        let left_status = Self::num_status(pa, pb);
                        best[left_status] = std::cmp::min(best[left_status], pa as i32 - pb as i32);
                        left += 1;

                        pa += (s.as_bytes()[left as usize] as char == a) as usize;
                        pb += (s.as_bytes()[left as usize] as char == b) as usize;
                    }

                    let right_status = Self::num_status(ca, cb);
                    if best[right_status ^ 0b10] != i32::MAX { ans = std::cmp::max(ans, ca as i32 - cb as i32 - best[right_status ^ 0b10]); }
                }
            }
        }
        ans
    }
}

fn main() {
    assert_eq!(-1, Solution::max_difference("12233".to_string(), 4));
    assert_eq!(1, Solution::max_difference("1122211".to_string(), 3));
    assert_eq!(-1, Solution::max_difference("110".to_string(), 3));
    assert_eq!(1, Solution::max_difference("44114402".to_string(), 7));
    assert_eq!(1, Solution::max_difference("0443143323".to_string(), 7));
    assert_eq!(3, Solution::max_difference("04333232302314".to_string(), 5));
    assert_eq!(-1, Solution::max_difference("2222130".to_string(), 2));
}
