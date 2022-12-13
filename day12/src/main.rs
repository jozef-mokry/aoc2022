use std::collections::{HashSet, VecDeque};
fn main() {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut end = (0, 0);
    let mut queue = VecDeque::new();
    for (i, line) in std::io::stdin().lines().enumerate() {
        let mut row: Vec<u8> = vec![];
        for (j, chr) in line.unwrap().as_bytes().iter().enumerate() {
            row.push(match &chr {
                // remove "| b'a'" for part1
                b'S' | b'a' => {
                    queue.push_back(((i, j), 0));
                    0
                }
                b'E' => {
                    end = (i, j);
                    25
                }
                b'a'..=b'z' => chr - b'a',
                _ => unreachable!(),
            })
        }
        grid.push(row);
    }
    let mut seen = HashSet::new();
    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == end {
            println!("{steps}");
            return;
        }
        if seen.contains(&(x, y)) {
            continue;
        }
        seen.insert((x, y));

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (x, y) = (x as isize, y as isize);
            let (xx, yy) = ((x + dx) as usize, (y + dy) as usize);
            if x + dx >= 0
                && x + dx < grid.len() as isize
                && y + dy >= 0
                && y + dy < grid[0].len() as isize
                && grid[xx][yy] <= grid[x as usize][y as usize] + 1
            {
                queue.push_back(((xx, yy), steps + 1));
            }
        }
    }
    println!("done");
}
