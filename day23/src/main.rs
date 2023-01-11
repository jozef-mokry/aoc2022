use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut elves: HashSet<(isize, isize)> = HashSet::new();
    for (i, line) in std::io::stdin().lines().enumerate() {
        let line = line.unwrap();
        for (j, byte) in line.bytes().enumerate() {
            if byte == b'#' {
                elves.insert((i as isize, j as isize));
            }
        }
    }

    let mut dirs = VecDeque::from(["N", "S", "W", "E"]);
    // print(&elves);
    for r in 1.. {
        if !round(&mut elves, &mut dirs) {
            println!("no change in round {r}");
            break;
        }

        if r == 10 {
            println!("round 10, unused space: {}", area(&elves));
        }

        // println!("Round {r}:");
        // print(&elves);
    }
}

fn area(elves: &HashSet<(isize, isize)>) -> isize {
    let x_min = elves.iter().min_by_key(|e| e.0).unwrap().0;
    let x_max = elves.iter().max_by_key(|e| e.0).unwrap().0;
    let y_min = elves.iter().min_by_key(|e| e.1).unwrap().1;
    let y_max = elves.iter().max_by_key(|e| e.1).unwrap().1;

    (x_max - x_min + 1) * (y_max - y_min + 1) - elves.len() as isize
}

fn print(elves: &HashSet<(isize, isize)>) {
    let x_min = elves.iter().min_by_key(|e| e.0).unwrap().0;
    let x_max = elves.iter().max_by_key(|e| e.0).unwrap().0;
    let y_min = elves.iter().min_by_key(|e| e.1).unwrap().1;
    let y_max = elves.iter().max_by_key(|e| e.1).unwrap().1;
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn round(elves: &mut HashSet<(isize, isize)>, dirs: &mut VecDeque<&str>) -> bool {
    let mut proposed = HashMap::new();
    for elve in elves.iter() {
        if alone(elve, elves) {
            continue;
        }
        for &dir in dirs.iter() {
            if let Some(new_pos) = propose(elves, elve, dir) {
                proposed.entry(new_pos).and_modify(|e| *e += 1).or_insert(1);
                break;
            }
        }
    }

    let mut new_set = HashSet::new();
    for elve in elves.iter() {
        let mut inserted = false;
        for &dir in dirs.iter() {
            if alone(elve, elves) {
                break;
            }
            if let Some(new_pos) = propose(elves, elve, dir) {
                if proposed.get(&new_pos) == Some(&1) {
                    new_set.insert(new_pos);
                    inserted = true;
                }
                break;
            }
        }
        if !inserted {
            new_set.insert(*elve);
        }
    }

    assert!(new_set.len() == elves.len());
    let change = new_set != *elves;
    *elves = new_set;
    let first = dirs.pop_front().unwrap();
    dirs.push_back(first);
    return change;
}

fn propose(
    elves: &HashSet<(isize, isize)>,
    &(x, y): &(isize, isize),
    dir: &str,
) -> Option<(isize, isize)> {
    let check = match dir {
        "N" => [(x - 1, y), (x - 1, y - 1), (x - 1, y + 1)],
        "S" => [(x + 1, y), (x + 1, y - 1), (x + 1, y + 1)],
        "W" => [(x, y - 1), (x - 1, y - 1), (x + 1, y - 1)],
        "E" => [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)],
        _ => unreachable!(),
    };

    if check.iter().all(|pos| !elves.contains(pos)) {
        return Some(check[0]);
    }
    None
}

fn alone(&(x, y): &(isize, isize), elves: &HashSet<(isize, isize)>) -> bool {
    for xx in (x - 1)..=(x + 1) {
        for yy in (y - 1)..=(y + 1) {
            if xx == x && yy == y {
                continue;
            }
            if elves.contains(&(xx, yy)) {
                return false;
            }
        }
    }
    return true;
}
