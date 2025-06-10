struct Solution;

impl Solution {
    fn quick_check(a: &u8, b: &u8) -> bool {
        if *a == b'(' && *b == b')' { true } 
        else if *a == b'{' && *b == b'}' { true } 
        else if *a == b'[' && *b == b']' { true } 
        else { false }
    }

    fn is_valid(s: String) -> bool {
        let mut v = Vec::with_capacity(s.len());

        for i in s.as_bytes() {
            if v.is_empty() { v.push(i); }
            else {
                if Self::quick_check(v.last().unwrap(), i) { v.pop(); }
                else { v.push(i); }
            }
        }

        v.is_empty()
    }
}

fn main() {
    let a = String::from("()[{{}}]");
    if Solution::is_valid(a) { println!("YES"); }
    else { println!("NO"); }
}
