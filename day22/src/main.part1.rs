fn main() {
    let mut grid = vec![];
    for line in std::io::stdin().lines() {
        let mut line = line.unwrap();
        while line.len() < 150 {
            line.push(' ');
        }
        grid.push(line);
    }
    let instrs = grid.pop().unwrap();
    grid.pop(); // empty line

    let mut steps = instrs
        .split(['L', 'R'])
        .map(|x| x.parse::<isize>().unwrap());
    let mut turns = instrs
        .split(|c: char| c.is_digit(10))
        .filter(|x| x.len() > 0);
    let mut x = 0;
    let mut y = 50;
    let mut dir = 0;
    while let Some(steps) = steps.next() {
        (x, y) = go(x, y, dir, steps, &grid);
        if let Some(turn) = turns.next() {
            dir = new_dir(dir, turn);
        }
    }
    println!("{}", (x + 1) * 1000 + (y + 1) * 4 + dir as isize);
}

fn new_dir(dir: usize, turn: &str) -> usize {
    match turn {
        "L" => (dir - 1) % 4,
        "R" => (dir + 1) % 4,
        _ => unreachable!(),
    }
}

fn go(x: isize, y: isize, dir: usize, steps: isize, grid: &[String]) -> (isize, isize) {
    let (dx, dy) = match dir {
        0 => (0, 1),
        1 => (1, 0),
        2 => (0, -1),
        3 => (-1, 0),
        _ => unreachable!(),
    };
    let mut x = x;
    let mut y = y;
    for _ in 0..steps {
        let mut cand_x = x;
        let mut cand_y = y;
        loop {
            cand_x += dx;
            cand_y += dy;
            if cand_x < 0 {
                cand_x += grid.len() as isize;
            } else if cand_x >= grid.len() as isize {
                cand_x -= grid.len() as isize;
            }
            if cand_y < 0 {
                cand_y += grid[cand_x as usize].len() as isize;
            } else if cand_y >= grid[cand_x as usize].len() as isize {
                cand_y -= grid[cand_x as usize].len() as isize;
            }
            if grid[cand_x as usize].as_bytes()[cand_y as usize] == b'#' {
                return (x, y);
            }
            if grid[cand_x as usize].as_bytes()[cand_y as usize] == b'.' {
                break;
            }
        }
        if (cand_x, cand_y) == (x, y) {
            return (x, y);
        }
        (x, y) = (cand_x, cand_y);
    }
    (x, y)
}
