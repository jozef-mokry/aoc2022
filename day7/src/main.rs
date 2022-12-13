fn main() {
    let mut stack: Vec<usize> = vec![0];
    let mut sizes: Vec<usize> = vec![];
    for line in std::io::stdin().lines() {
        match &line.unwrap().split_whitespace().collect::<Vec<_>>()[..] {
            &["$", "cd", "/"] | &["$", "ls"] | &["dir", _] => {}
            &["$", "cd", ".."] => {
                let size = stack.pop().unwrap();
                *stack.last_mut().unwrap() += size;
                sizes.push(size);
            }
            &["$", "cd", _] => stack.push(0),
            &[size, _] => *stack.last_mut().unwrap() += size.parse::<usize>().unwrap(),
            _ => unreachable!(),
        }
    }
    while stack.len() >= 2 {
        let size = stack.pop().unwrap();
        sizes.push(size);
        *stack.last_mut().unwrap() += size;
    }
    let need: usize = stack[0] + 30_000_000 - 70_000_000;
    println!("{}", sizes.iter().filter(|&&s| s <= 100_000).sum::<usize>());
    println!("part2: {:?}", sizes.iter().filter(|&&s| s >= need).min());
}
