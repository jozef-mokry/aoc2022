use std::io;
fn day5() {
    //
    //     [H]         [H]         [V]
    //     [V]         [V] [J]     [F] [F]
    //     [S] [L]     [M] [B]     [L] [J]
    //     [C] [N] [B] [W] [D]     [D] [M]
    // [G] [L] [M] [S] [S] [C]     [T] [V]
    // [P] [B] [B] [P] [Q] [S] [L] [H] [B]
    // [N] [J] [D] [V] [C] [Q] [Q] [M] [P]
    // [R] [T] [T] [R] [G] [W] [F] [W] [L]

    let mut stacks = vec![
        vec!["G", "P", "N", "R"],
        vec!["H", "V", "S", "C", "L", "B", "J", "T"],
        vec!["L", "N", "M", "B", "D", "T"],
        vec!["B", "S", "P", "V", "R"],
        vec!["H", "V", "M", "W", "S", "Q", "C", "G"],
        vec!["J", "B", "D", "C", "S", "Q", "W"],
        vec!["L", "Q", "F"],
        vec!["V", "F", "L", "D", "T", "H", "M", "W"],
        vec!["F", "J", "M", "V", "B", "P", "L"],
    ];
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let lines = io::stdin().lines();
    for line in lines {
        let line = line.unwrap();
        if !line.starts_with("move") {
            continue;
        }
        // move 3 from 3 to 7
        let words: Vec<&str> = line.split_whitespace().collect();
        let count: i32 = words[1].parse().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        let mut tmp = vec![];
        for _ in 0..count {
            let val = stacks[from].pop().unwrap();
            tmp.push(val);
        }

        for val in tmp.into_iter().rev() {
            stacks[to].push(val);
        }
    }

    for mut stack in stacks {
        if let Some(val) = stack.pop() {
            print!("{val}");
        }
    }
}

fn main() {
    day5();
}
