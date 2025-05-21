struct Solution;

impl Solution {
    fn bs(ns: &[i32], x: i32, start: usize, n: usize) -> usize {
        let mut left = start; let mut right = n-1; let mut mid; let mut c=n;
        // println!("left, right = ({}, {})", left,right);

        while left <= right {
            mid = left+((right-left)>>1);
            // println!("mid: {}",mid);

            if ns[mid] > x { c=mid; left=mid+1; }
            else { right = mid-1; }
        }
        return c;
    }

    fn next_permutation(ns: &mut Vec<i32>) {
        let mut tempo;
        let n = ns.len();

        if n <= 1 { return; }

        let mut i = n-2;
        let mut j = n-1;

        while i > 0 && ns[i] >= ns[i+1] { i -= 1; } 
        if i == 0 && ns[i] > ns[i+1] {
            // println!("HI");
            i=0;
            while i < j {
                tempo = ns[i];
                ns[i] = ns[j];
                ns[j] = tempo;

                i += 1;
                j -= 1;
            }
        } else {
            let a = Self::bs(ns, ns[i], i+1,n);

            if a != n {
                tempo = ns[a];
                ns[a] = ns[i];
                ns[i] = tempo;
            }
            // println!("{}, {}", i+1,j);
            while i+1 < j {
                // for z in 0..ns.len() { println!("{}", ns[z]); }
                tempo = ns[i+1];
                ns[i+1] = ns[j];
                ns[j] = tempo;

                i += 1;
                j -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn solve() {
        let mut a = vec![5,1,1];
        for z in 0..a.len() { println!("{}", a[z]); }
        Solution::next_permutation(&mut a);
        assert_eq!(a, vec![1,1,5]);
    }

    #[test]
    fn solve_2() {
        let mut a = vec![3,2,1];
        for z in 0..a.len() { println!("{}", a[z]); }
        Solution::next_permutation(&mut a);
        assert_eq!(a, vec![1,2,3]);
    }

    #[test]
    fn solve_3() {
        let mut a = vec![1,1,5];
        for z in 0..a.len() { println!("{}", a[z]); }
        Solution::next_permutation(&mut a);
        assert_eq!(a, vec![1,5,1]);
    }
}

fn main() {
    let mut a = vec![1,3,2];
    Solution::next_permutation(&mut a);
    println!("{:?}", a);
}
