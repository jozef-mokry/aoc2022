use std::collections::HashMap;
fn main() {
    let mut graph = HashMap::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (node, val) = line.split_once(": ").unwrap();
        graph.insert(
            node.to_owned(),
            val.split(' ').map(String::from).collect::<Vec<_>>(),
        );
    }
    println!("part1: {}", dfs("root", &graph, &mut HashMap::new()));
}

fn dfs(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    mem: &mut HashMap<String, isize>,
) -> isize {
    if mem.contains_key(node) {
        return mem[node];
    }
    let ans = match &graph[node][..].as_ref() {
        &[num] => num.parse::<isize>().unwrap(),
        &[a, op, b] if op == "*" => dfs(&a, graph, mem) * dfs(&b, graph, mem),
        &[a, op, b] if op == "+" => dfs(&a, graph, mem) + dfs(&b, graph, mem),
        &[a, op, b] if op == "-" => dfs(&a, graph, mem) - dfs(&b, graph, mem),
        &[a, op, b] if op == "/" => dfs(&a, graph, mem) / dfs(&b, graph, mem),
        _ => unreachable!(),
    };
    mem.insert(node.to_owned(), ans);
    ans
}
