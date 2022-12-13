fn main() {
    let mut x: isize = 1;
    let (mut delay2, mut delay1) = (0, 0);
    let mut ans = 0;
    let mut lines = std::io::stdin().lines();
    for cycle in 1..=240 {
        x += delay1;
        delay1 = delay2;
        delay2 = 0;
        let pos = (cycle - 1) % 40;
        if pos == 0 {
            println!("");
        }
        let px = if (pos - x).abs() <= 1 { '#' } else { '.' };
        print!("{px}");
        if cycle % 40 == 20 {
            ans += x * cycle;
        }
        if delay1 != 0 {
            continue;
        }
        if let Some(Ok(line)) = lines.next() {
            match line.split_whitespace().collect::<Vec<_>>()[..] {
                ["noop"] => {}
                ["addx", v] => delay2 = v.parse::<isize>().unwrap(),
                _ => unreachable!(),
            }
        }
    }
    println!("{ans}");
}
