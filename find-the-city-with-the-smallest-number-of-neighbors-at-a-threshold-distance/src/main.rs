struct Solution;
impl Solution {
    fn convert(n: i32, e: &Vec<Vec<i32>>) -> Vec<Vec<(i32, i32)>> {
        let mut v = vec![Vec::new(); n as usize];

        for i in e {
            v[i[0] as usize].push((i[1], i[2]));
            v[i[1] as usize].push((i[0], i[2]));
        }

        v
    }

    fn dijkstra(n: i32, start: i32, graph: &Vec<Vec<(i32, i32)>>) -> Vec<i32> {
        let mut set = std::collections::BTreeSet::new();
        let mut distance = vec![i32::MAX; n as usize];

        distance[start as usize] = 0;
        set.insert((0, start));

        while !set.is_empty() {
            let tp = set.first().unwrap().clone();
            set.pop_first();

            for v in &graph[tp.1 as usize] {
                if distance[v.0 as usize] > distance[tp.1 as usize] + v.1 {
                    set.remove(&(distance[v.0 as usize], v.0));
                    distance[v.0 as usize] = distance[tp.1 as usize] + v.1;
                    set.insert((distance[v.0 as usize], v.0));
                }
            }
        }
        distance
    }

    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let v = Self::convert(n, &edges);
        let (mut tp, mut res)= (i32::MAX, 0);
        let mut hold_temp;

        for i in 0..n {
            hold_temp = Self::dijkstra(n, i, &v).iter().filter(|&x| *x != i32::MAX && *x <= distance_threshold).count() as i32;
            if tp >= hold_temp { tp = hold_temp; res = i; }
        }
        res
    }
}

fn main() {
    assert_eq!(Solution::find_the_city(4, vec![ vec![0,1,3], vec![1,2,1], vec![1,3,4], vec![2,3,1]], 4), 3);
    assert_eq!(Solution::find_the_city(5, vec![ vec![0,1,2], vec![0,4,8], vec![1,2,3], vec![1,4,2], vec![2,3,1], vec![3,4,1] ], 2), 0);
}
