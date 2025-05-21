struct Solution;

impl Solution {
    fn recur(v: &mut Vec<Vec<String>>, subv: &mut Vec<i32>, n: i32) {
        if subv.len() == n as usize {
            let mut take_sub = Vec::new();

            for i in 0..n {
                let c = (0..n)
                .map(|x| if x == subv[i as usize] { 'Q' } else { '.' } )
                .collect::<String>();

                take_sub.push(c);
            }
            v.push(take_sub);
        }

        for i in 0..n {
            if subv.contains(&i) { continue; }

            let mut is_valid = true;

            for j in 0..subv.len() {
                if (subv[j] - i).abs() == subv.len() as i32 - j as i32 {
                    is_valid = false;
                    break;
                }
            }

            if !is_valid { continue; }

            subv.push(i);
            Self::recur(v,subv,n);
            subv.pop();
        }
    }

    fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut v = Vec::new();
        Self::recur(&mut v, &mut Vec::new(), n);

        v
    }
}

fn main() {
    println!("{:?}", Solution::solve_n_queens(4));
}
