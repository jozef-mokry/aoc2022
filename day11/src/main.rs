#[derive(Debug, Clone)]
struct Monkey {
    divisor: isize,
    items: Vec<isize>,
    if_true: usize,
    if_false: usize,
    op: Operation,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(Val),
    Mult(Val),
}

#[derive(Debug, Clone)]
enum Val {
    Old,
    Const(isize),
}

impl Operation {
    fn eval(&self, x: isize) -> isize {
        match self {
            Operation::Add(Val::Old) => x + x,
            Operation::Mult(Val::Old) => x * x,
            Operation::Add(Val::Const(y)) => x + y,
            Operation::Mult(Val::Const(y)) => x * y,
        }
    }
}

// Monkey 0:
//   Starting items: 54, 89, 94
//   Operation: new = old * 7
//   Test: divisible by 17
//     If true: throw to monkey 5
//     If false: throw to monkey 3
fn parse_monkey<T: std::io::BufRead>(lines: &mut std::io::Lines<T>) -> Option<Monkey> {
    assert!(lines.next()?.ok()?.starts_with("Monkey "));
    let items: Vec<isize> = lines
        .next()?
        .ok()?
        .split_once(": ")?
        .1
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect();
    // TODO: operation
    let op = {
        let line = lines.next()?.ok()?;

        let (op, val) = line.split_once("new = old ")?.1.split_once(" ")?;
        let val = match val {
            "old" => Val::Old,
            x => Val::Const(x.parse().ok()?),
        };
        match op {
            "+" => Operation::Add(val),
            "*" => Operation::Mult(val),
            _ => unreachable!(),
        }
    };

    let divisor: isize = lines.next()?.ok()?.rsplit_once(" ")?.1.parse().ok()?;
    let if_true: usize = lines.next()?.ok()?.rsplit_once(" ")?.1.parse().ok()?;
    let if_false: usize = lines.next()?.ok()?.rsplit_once(" ")?.1.parse().ok()?;
    lines.next(); // last empty line

    Some(Monkey {
        items,
        divisor,
        if_true,
        if_false,
        op,
    })
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut monkeys = (0..8)
        .map(|_| parse_monkey(&mut lines))
        .collect::<Option<Vec<_>>>()
        .unwrap();
    let modulo: isize = monkeys.iter().map(|m| m.divisor).product();
    let rounds = 10_000; // 20 for part1
    let mut counts = vec![0; monkeys.len()];
    for _ in 1..=rounds {
        for i in 0..(monkeys.len()) {
            let monkey = monkeys[i].clone();
            for item in monkey.items {
                // part1: let new_item = monkey.op.eval(item) / 3;
                let new_item = monkey.op.eval(item) % modulo;
                counts[i] += 1;
                if new_item % monkey.divisor == 0 {
                    monkeys[monkey.if_true].items.push(new_item);
                } else {
                    monkeys[monkey.if_false].items.push(new_item);
                }
            }
            monkeys[i].items.clear();
        }
    }
    counts.sort();
    counts.reverse();
    println!("{:?}", counts[0..2].iter().product::<isize>());
}
