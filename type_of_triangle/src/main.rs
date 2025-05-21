struct Solution;

impl Solution {
    pub fn triangle_type(num: Vec<i32>) -> String {
        if num[0] + num[1] <= num[2] || num[1] + num[2] <= num[0] || num[0] + num[2] <= num[1] { "none".to_string() }
        else if num[0] == num[1] && num[1] == num[2] { "equilateral".to_string() }
        else if num[0] == num[1] || num[1] == num[2] || num[2] == num[0] { "isosceles".to_string() } 
        else { "scalene".to_string() }
    }
}

fn main() {
    println!("{}", Solution::triangle_type(vec![1,2,100000]));
}
