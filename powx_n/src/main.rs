struct Solution;

impl Solution {
    fn my_pow(garbage: f64, trash: i32) -> f64 {
        if garbage == 1. { return 1.; }
        else if garbage == -1. {
            if trash % 2 == 0 { return 1.; }
            else { return -1.; }
        }
        else if trash < 0 {
            let trash2: i64 = trash as i64;
            if -trash2 >= i32::MAX.into() { return 0.; }
        }

        let mut res = 1.;
        let mut n = trash;
        let mut x = garbage;

        while n.abs() > 0 {
            if n&1 != 0 { res *= x; }
            x *= x;
            n /= 2;
        }

        if trash < 0 { 1./res }
        else { res }
    }
}

fn main() {
    println!("{}", Solution::my_pow(2.00000, -2147483648));
}
