use std::collections::{BTreeSet, VecDeque};
use std::ops::Bound::{Included, Unbounded};
fn main() {
    let mut storms = BTreeSet::new();
    let (mut rows, mut cols) = (0, 0);
    for (i, line) in std::io::stdin().lines().enumerate() {
        rows += 1;
        cols = line.as_ref().unwrap().len();
        for (j, byte) in line.unwrap().bytes().enumerate() {
            if let b'^' | b'v' | b'>' | b'<' = byte {
                storms.insert((i, j, byte));
            }
        }
    }

    let goal = (rows - 1, cols - 2);
    let start: (usize, usize) = (0, 1);

    let mut storms_at_time: Vec<BTreeSet<_>> = vec![storms];

    let to_goal = fastest(start, goal, rows, cols, 0, &mut storms_at_time);
    println!("to_goal: {to_goal}");
    let to_start = fastest(goal, start, rows, cols, to_goal, &mut storms_at_time);
    println!("and back to start: {to_start}");
    let to_goal_again = fastest(start, goal, rows, cols, to_start, &mut storms_at_time);
    println!("ans: {to_goal_again}");
}

fn fastest(
    start: (usize, usize),
    goal: (usize, usize),
    rows: usize,
    cols: usize,
    start_time: usize,
    storms_at_time: &mut Vec<BTreeSet<(usize, usize, u8)>>,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front((start_time, start));
    let mut seen: BTreeSet<_> = BTreeSet::new();
    while let Some((time, (r, c))) = queue.pop_front() {
        if (r, c) == goal {
            return time;
        }
        if seen.contains(&(time, (r, c))) {
            continue;
        }
        seen.insert((time, (r, c)));
        if (r, c) != start && (r == 0 || r == rows - 1 || c == 0 || c == cols - 1) {
            // hit a wall #
            continue;
        }
        match storms_at_time[time]
            .range((Included((r, c, 0)), Unbounded))
            .next()
        {
            Some(&(rr, cc, _)) if (rr, cc) == (r, c) => continue, // hit a storm
            _ => {}
        }

        if storms_at_time.len() == time + 1 {
            let new_storms: BTreeSet<_> = storms_at_time[time]
                .iter()
                .map(|&(r, c, dir)| match (dir, r, c) {
                    (b'>', r, c) if c == cols - 2 => (r, 1, b'>'),
                    (b'>', r, c) => (r, c + 1, b'>'),
                    (b'<', r, c) if c == 1 => (r, cols - 2, b'<'),
                    (b'<', r, c) => (r, c - 1, b'<'),
                    (b'^', r, c) if r == 1 => (rows - 2, c, b'^'),
                    (b'^', r, c) => (r - 1, c, b'^'),
                    (b'v', r, c) if r == rows - 2 => (1, c, b'v'),
                    (b'v', r, c) => (r + 1, c, b'v'),
                    _ => unreachable!(),
                })
                .collect();
            storms_at_time.push(new_storms);
        }

        for (dx, dy) in [(0isize, 1), (0, -1), (1, 0), (-1, 0), (0, 0)] {
            let (r, c) = (r as isize, c as isize);
            if r + dx >= 0 && c + dy >= 0 {
                queue.push_back((time + 1, ((r + dx) as usize, (c + dy) as usize)));
            }
        }
    }
    unreachable!();
}
