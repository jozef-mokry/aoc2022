use std::collections::{HashMap, VecDeque};

#[must_use]
fn set_add(set: usize, x: usize) -> usize {
    set | (1 << x)
}

fn set_contains(set: &usize, x: usize) -> bool {
    (set & (1 << x)) > 0
}

fn main() {
    let mut name2idx = HashMap::new();
    let mut idx = |name: &str| {
        let len = name2idx.len();
        *name2idx.entry(name.to_string()).or_insert(len)
    };
    let mut rates = vec![];
    let mut edges = vec![];
    let mut nonzeros = vec![];
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        // Valve NQ has flow rate=0; tunnels lead to valves SU, XD
        let parts: Vec<_> = line.split([' ', ';', '=', ',']).collect();
        let from = idx(parts[1]);
        let rate: usize = parts[5].parse().unwrap();
        let neighbours: Vec<_> = parts[11..]
            .into_iter()
            .step_by(2)
            .map(|x| idx(*x))
            .collect();
        while from >= rates.len() {
            rates.push(0);
            edges.push(vec![]);
        }
        rates[from] = rate;
        edges[from] = neighbours;
        if rate > 0 {
            nonzeros.push(from);
        }
    }

    // compute all distances
    let mut dist = vec![vec![9999; rates.len()]; rates.len()];
    for i in 0..rates.len() {
        for &j in &edges[i] {
            dist[i][j] = 1;
        }
    }
    for k in 0..rates.len() {
        for i in 0..rates.len() {
            for j in 0..rates.len() {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    let start = idx("AA");
    let used = set_add(0, start);
    queue.push_back(((start, start), (26isize, 26isize), used, 0));
    let mut ans = 0;
    let mut visited_states = 0;
    while let Some(((a, b), (time_a, time_b), used, flow)) = queue.pop_front() {
        visited_states += 1;
        if time_a == 0 && time_b == 0 {
            ans = ans.max(flow);
            continue;
        }
        if time_a >= time_b {
            for &v in &nonzeros {
                if set_contains(&used, v) {
                    continue;
                }
                let new_time = time_a - dist[a][v] - 1;
                if new_time <= 0 {
                    continue;
                }
                let new_flow = flow + (new_time as usize) * rates[v];
                let new_used = set_add(used, v);
                queue.push_back(((v, b), (new_time, time_b), new_used, new_flow));
            }
            queue.push_back(((999, b), (0, time_b), used, flow)); // stop moving
        } else {
            for &v in &nonzeros {
                if set_contains(&used, v) {
                    continue;
                }

                let new_time = time_b - dist[b][v] - 1;
                if new_time <= 0 {
                    continue;
                }
                let new_flow = flow + (new_time as usize) * rates[v];
                let new_used = set_add(used, v);
                queue.push_back(((a, v), (time_a, new_time), new_used, new_flow));
            }
            queue.push_back(((a, 0), (time_a, 0), used, flow)); // stop moving
        }
    }
    println!("{ans} {visited_states}");
}
