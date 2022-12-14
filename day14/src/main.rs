use std::collections::HashSet;

fn draw(grid: &mut HashSet<(isize, isize)>, from: (isize, isize), to: (isize, isize)) {
    let norm = |x: isize| if x == 0 { 0 } else { x / x.abs() };
    let dir = (norm(to.0 - from.0), norm(to.1 - from.1));
    let mut curr = from;
    grid.insert(curr);
    while curr != to {
        curr = (curr.0 + dir.0, curr.1 + dir.1);
        grid.insert(curr);
    }
}

fn update(curr: &mut (isize, isize), grid: &HashSet<(isize, isize)>, floor: isize) -> bool {
    if curr.1 == floor - 1 {
        // on the floor
        return false;
    }
    if !grid.contains(&(curr.0, curr.1 + 1)) {
        curr.1 += 1;
        return true;
    }

    if !grid.contains(&(curr.0 - 1, curr.1 + 1)) {
        curr.0 -= 1;
        curr.1 += 1;
        return true;
    }
    if !grid.contains(&(curr.0 + 1, curr.1 + 1)) {
        curr.0 += 1;
        curr.1 += 1;
        return true;
    }
    return false;
}

fn main() {
    let mut grid = HashSet::new();
    let mut lowest = 0;
    for line in std::io::stdin().lines() {
        let mut prev_point = None;
        let line = line.unwrap();
        for point in line.split(" -> ") {
            let point: Vec<isize> = point
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            let point = (point[0], point[1]);
            lowest = lowest.max(point.1);
            if let Some(prev_point) = prev_point {
                draw(&mut grid, prev_point, point);
            }
            prev_point = Some(point);
        }
    }

    let mut ans = 0;
    let part1 = true;
    loop {
        let mut curr = (500, 0);
        while update(&mut curr, &grid, lowest + 2) {}
        if part1 && curr.1 == lowest + 1 {
            // reached the floor
            println!("part1: {ans}");
            return;
        }
        ans += 1;
        if !part1 && curr == (500, 0) {
            println!("part2: {ans}");
            return;
        }
        draw(&mut grid, curr, curr);
    }
}
