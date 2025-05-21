struct Solution;

impl Solution {
    fn bin_search(n: usize, left: usize, v: &[i32], x: i32) -> usize {
        let mut l = left+1; let mut r = n-1; let mut mid; let mut c=n;

        if l == r { return l; }
        while l <= r {
            mid = l + ((r-l)>>1);

            if v[mid] < x { l = mid+1; }
            else if v[mid] > x { r = mid -1; }
            else { return mid; }

            if c != n { if (x-v[mid]).abs() < (x-v[c]).abs() { c = mid; } }
            else { c = mid; }
        }
        return c;
    }

    fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut ns = nums.clone();
        let mut a;
        let mut keep_min = 1000000;
        let mut keep_sum=0;
        let n = ns.len();
        ns.sort();

        println!("{:?}",ns);

        for i in 0..n-2 {
            for j in i+1..n-1 {
                println!("{}, {}", i,j);
                a = Self::bin_search(n,j,&ns,target-ns[i]-ns[j]);
                println!("{}",a);

                if a != n {
                    if keep_min > (target - ns[i] - ns[j] - ns[a]).abs() {
                        keep_min = (target - ns[i] - ns[j] - ns[a]).abs();
                        keep_sum = ns[i] + ns[j] + ns[a];
                    }
                }
            }
        }
        keep_sum
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_test() {
        let a = vec![-1,2,1,-4];
        let target = 0;

        assert_eq!(-1, super::Solution::three_sum_closest(a, target))
    }
}

fn main() {
    let a = vec![-1,2,1,-4];
    // let a = vec![1;4];
    // let a = vec![0,0,0];
    let target = 0;
    println!("{}", Solution::three_sum_closest(a, target));
}
