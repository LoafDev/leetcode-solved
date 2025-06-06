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

    fn dijkstra(n: i32, dt: i32, start: i32, graph: &Vec<Vec<(i32, i32)>>) -> Vec<i32> {
        let mut set = std::collections::BTreeSet::new();
        let mut distance = vec![-1; n as usize];

        distance[start as usize] = 0;
        set.insert((start, 0));

        while !set.is_empty() {
            let tp = set.first().unwrap().clone();
            set.pop_first();

            for v in &graph[tp.0 as usize] {
                if distance[v.0 as usize] < distance[tp.0 as usize] + v.1 && distance[tp.0 as usize] + v.1 <= dt {
                    set.remove(&(v.0, distance[v.0 as usize]));
                    distance[v.0 as usize] = distance[tp.0 as usize] + v.1;
                    set.insert((v.0, distance[v.0 as usize]));
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
            hold_temp = Self::dijkstra(n, distance_threshold, i, &v).iter().filter(|&x| *x != -1).count() as i32;
            if tp >= hold_temp { tp = hold_temp; res = i; }
        }
        res
    }
}

fn main() {
    
}
