struct Solution;

impl Solution {
    fn length_of_longest_substring_slow(s: String) -> i32 {
        if s.is_empty() { return 0; }

        let a = s.as_bytes();
        let mut v = vec![false; 1000];
        let mut l = 1;
        let mut maxn = 1;

        for i in 0..a.len()-1 {
            // print!("{}", a[i] as char);
            v[a[i] as usize] = true;
            for j in i+1..s.as_bytes().len() {
                if v[a[j] as usize] { v = vec![false; 1000]; maxn = std::cmp::max(l,maxn); l=1; break; }
                else { v[a[j] as usize] = true; l += 1; } // print!("{}", a[j] as char); l += 1; }
            }
            // println!("");
        }

        std::cmp::max(maxn,l)
    }

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
    println!("{}", Solution::length_of_longest_substring(String::from("ashdqw")));
}
