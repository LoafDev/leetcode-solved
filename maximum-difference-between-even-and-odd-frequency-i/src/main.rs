struct Solution;

const CHAR_INDEX: u8 = 97;
const ALPHABET_NUMS: usize = 26;
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut vec_chars = vec![0; ALPHABET_NUMS];
        let (mut max_odd, mut min_even) = (i32::MIN, i32::MAX);

        s.bytes().map(|i| (i % CHAR_INDEX) as usize).for_each(|i| vec_chars[i] += 1);

        for i in &vec_chars {
            if *i == 0 { continue; }
            else if i&1 != 0 { max_odd = std::cmp::max(max_odd, *i); }
            else { min_even = std::cmp::min(min_even, *i); }
        }

        max_odd - min_even
    }
}

fn main() {
    assert_eq!(3, Solution::max_difference("aaaaabbc".to_string()));
    assert_eq!(1, Solution::max_difference("abcabcab".to_string()));
}
