struct Solution;

impl Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut ns = nums.clone();
        let mut v = Vec::new();
        let mut j;
        let mut k;
        let mut tempo;

        ns.sort();
        for i in 0..n {
            if i > 0 && ns[i] == ns[i-1] { continue; }

            j = i+1;
            k = n-1;

            while j < k {
                tempo = ns[i] + ns[j] + ns[k];

                if tempo > 0 { k -= 1; }
                else if tempo < 0 { j += 1; }
                else {
                    v.push(vec![ns[i],ns[j],ns[k]]);
                    j += 1;

                    while ns[j] == ns[j-1] && j < k { j += 1; }
                }
            }
        }
        v
    }
}

fn main() {
    // let a = vec![-1,0,1,2,-1,-4];
    let a = vec![0,0,0,0];
    // let a = vec![0,1,1];
    let v = Solution::three_sum(a);
    for i in &v { println!("{:?}",i); }
}
