use std::collections::{BTreeSet, HashMap, VecDeque};
fn main() {
    let mut name2idx = HashMap::new();
    let mut idx = |name: &str| {
        let len = name2idx.len();
        *name2idx.entry(name.to_string()).or_insert(len)
    };
    let mut rates = vec![];
    let mut edges = vec![];
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
    }

    let mut queue = VecDeque::new();
    let start = idx("AA");
    #[derive(Default, Clone)]
    struct Best {
        flow: usize,
        time: isize,
        used: BTreeSet<usize>,
    };
    // let mut best = vec![Best::default(); rates.len()];
    queue.push_back((start, 30isize, BTreeSet::<usize>::new(), 0));
    let mut best = HashMap::<(usize, BTreeSet<usize>), usize>::new();
    let mut ans = 0;
    while let Some((curr, mut time, mut used, mut flow)) = queue.pop_front() {
        if time <= 0 {
            continue;
        }
        // println!("{time} {}", queue.len());
        if let Some(&prev_flow) = best.get(&(curr, used.clone())) {
            if prev_flow >= flow {
                continue;
            }
        }
        best.insert((curr, used.clone()), flow);
        ans = ans.max(flow);
        // if best[curr].flow >= flow && best[curr].time >= 0 && best[curr].used <= used {
        //     continue;
        // }

        let old_flow = flow;
        if rates[curr] > 0 && !used.contains(&curr) {
            used.insert(curr);
            time -= 1; // for turning on
            flow += (time as usize) * rates[curr];
        }
        // move to neighbours
        for &neighbour in &edges[curr] {
            queue.push_back((neighbour, time - 1, used.clone(), flow));
        }
        // move to neighbours
        for &neighbour in &edges[curr] {
            queue.push_back((neighbour, time, used.clone(), old_flow));
        }
    }
    println!("{ans}");
}
