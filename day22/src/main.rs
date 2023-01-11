const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;
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
    let mut dir = RIGHT;
    while let Some(steps) = steps.next() {
        (x, y, dir) = go(x, y, dir, steps, &grid);
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

fn go(x: isize, y: isize, dir: usize, steps: isize, grid: &[String]) -> (isize, isize, usize) {
    let mut x = x;
    let mut y = y;
    let mut dir = dir;
    for _ in 0..steps {
        // move on cube
        let (cand_x, cand_y, cand_dir) = cube_move(x, y, dir, grid);
        assert!(cand_x >= 0);
        assert!(cand_y >= 0);
        assert!(grid[cand_x as usize].as_bytes()[cand_y as usize] != b' ');
        if grid[cand_x as usize].as_bytes()[cand_y as usize] == b'#' {
            return (x, y, dir);
        }
        (x, y, dir) = (cand_x, cand_y, cand_dir);
    }
    (x, y, dir)
}

fn cube_move(x: isize, y: isize, dir: usize, grid: &[String]) -> (isize, isize, usize) {
    let (dx, dy) = match dir {
        RIGHT => (0, 1),
        DOWN => (1, 0),
        LEFT => (0, -1),
        UP => (-1, 0),
        _ => unreachable!(),
    };
    if x + dx >= 0
        && y + dy >= 0
        && x + dx < grid.len() as isize
        && y + dy < grid[0].len() as isize
        && grid[(x + dx) as usize].as_bytes()[(y + dy) as usize] != b' '
    {
        println!("{} {}", x + dx, y + dy);
        return (x + dx, y + dy, dir);
    }

    // we are falling off the grid
    // Shape:
    // .AB
    // .C
    // DE
    // F
    let face = match (x / 50, y / 50) {
        (0, 1) => 'A',
        (0, 2) => 'B',
        (1, 1) => 'C',
        (2, 0) => 'D',
        (2, 1) => 'E',
        (3, 0) => 'F',
        _ => unreachable!(),
    };

    match (face, dir) {
        ('A', LEFT) => (149 - x, 0, RIGHT),
        ('A', UP) => (150 + y - 50, 0, RIGHT),
        ('B', UP) => (199, y - 100, UP),
        ('B', DOWN) => (50 + y - 100, 99, LEFT),
        ('B', RIGHT) => (149 - x, 99, LEFT),
        ('C', RIGHT) => (49, 100 + x - 50, UP),
        ('C', LEFT) => (100, x - 50, DOWN),
        ('D', LEFT) => (49 - (x - 100), 50, RIGHT),
        ('D', UP) => (50 + y, 50, RIGHT),
        ('E', RIGHT) => (49 - x + 100, 149, LEFT),
        ('E', DOWN) => (150 + y - 50, 49, LEFT),
        ('F', LEFT) => (0, 50 + x - 150, DOWN),
        ('F', RIGHT) => (149, 50 + x - 150, UP),
        ('F', DOWN) => (0, 100 + y, DOWN),
        _ => unreachable!("{face:?} {dir:?}"),
    }
}
