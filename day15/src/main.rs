fn get_interval(
    (x, y, xx, yy): (isize, isize, isize, isize),
    at_y: isize,
) -> Option<(isize, isize)> {
    let dist = (x - xx).abs() + (y - yy).abs();
    let extra = dist - (y - at_y).abs();
    if extra < 0 {
        return None;
    }
    return Some((x - extra, x + extra));
}
fn main() {
    let mut pairs = vec![];
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        // Sensor at x=2899860, y=3122031: closest beacon is at x=2701269, y=3542780
        let parts: Vec<_> = line.split(['=', ' ', ',', ':']).collect();
        let (x, y, xx, yy) = (parts[3], parts[6], parts[13], parts[16]);
        let (x, y, xx, yy) = (
            x.parse::<isize>().unwrap(),
            y.parse::<isize>().unwrap(),
            xx.parse::<isize>().unwrap(),
            yy.parse::<isize>().unwrap(),
        );
        pairs.push(((x, y), (xx, yy)));
    }
    for at_y in 0..=4_000_000 {
        if let Some((x, y)) = unvisited(&pairs, at_y, 0, 4_000_000) {
            println!("{}", x * 4_000_000 + y);
        }
    }
}

fn unvisited(
    pairs: &Vec<((isize, isize), (isize, isize))>,
    at_y: isize,
    start: isize,
    end: isize,
) -> Option<(isize, isize)> {
    let mut intervals = vec![];
    for &((x, y), (xx, yy)) in pairs {
        if let Some((from, to)) = get_interval((x, y, xx, yy), at_y) {
            intervals.push((from, to));
        }
        if yy == at_y {
            intervals.push((xx, xx));
        }
    }
    intervals.sort();

    let mut next_uncovered = start;
    for (from, to) in intervals {
        if next_uncovered < from {
            return Some((next_uncovered, at_y));
        }
        next_uncovered = std::cmp::max(to + 1, next_uncovered);
    }
    if next_uncovered <= end {
        return Some((next_uncovered, at_y));
    }
    return None;
}
