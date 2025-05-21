struct Solution;

impl Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        let maxn = height.iter().max();
        let mut second_maxn = height.iter().filter(|x| x < &maxn.unwrap()).max();

        if second_maxn.is_none() { second_maxn = maxn; }

        let mut m = Vec::new();
        let mut n = Vec::new();

        for i in 0..height.len() {
            if height[i as usize] == *maxn.unwrap() { m.push(i); }
            if height[i as usize] == *second_maxn.unwrap() { n.push(i); }
        }

        /* let (a,b) = (
            (*n.last().unwrap() as i32 - *m.first().unwrap() as i32).abs(),
            (*m.last().unwrap() as i32 - *n.first().unwrap() as i32).abs()
        );

        println!("{}, {}", a,b); */

        let a = std::cmp::max(
                (*n.last().unwrap() as i32 - *m.first().unwrap() as i32).abs(),
                (*m.last().unwrap() as i32 - *n.first().unwrap() as i32).abs()
        ) * second_maxn.unwrap();
        let b = (m.last().unwrap() - m.first().unwrap()) as i32 * maxn.unwrap();
        let c = (n.last().unwrap() - n.first().unwrap()) as i32 * second_maxn.unwrap();

        // println!("{:?}, {:?}, {}, {}, {}", m.last(), m.first(), a,b,c);
        std::cmp::max(a.max(b),c)
    }
}

// optimize for exercise?
//
// how to?
//
// height = min(h[h1], h[h2])
// length = abs(h1 - h2)
// max of height * length?
//
// height = h[h1]
// => area = h2*h1 - h1^2
// height = h2
// => area = h1*h2 - h2^2

fn main() {
    println!("{}", Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
    println!("{}", Solution::max_area(vec![1,1]));
    println!("{}", Solution::max_area(vec![1,2,1]));
}
