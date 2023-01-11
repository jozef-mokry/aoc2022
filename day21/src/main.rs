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
    graph.get_mut("root").unwrap()[1] = "-".to_owned();

    let mut start: isize = 3146208344851;
    let mut end: isize = start * 2;
    while start < end {
        let humn = start + (end - start) / 2;
        graph.insert("humn".to_owned(), vec![humn.to_string()]);
        let ans = dfs("root", &graph, &mut HashMap::new());
        println!("{humn}:{ans}");
        if ans == 0 {
            break;
        }
        if ans > 0 {
            start = humn + 1;
        } else {
            end = humn - 1;
        }
    }
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
