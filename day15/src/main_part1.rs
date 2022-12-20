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
    let mut intervals = vec![];
    let at_y = 2_000_000;
    // let at_y = 10;
    let mut beacons_at_y = vec![];
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
        if yy == at_y {
            beacons_at_y.push(xx);
        }
        if let Some((from, to)) = get_interval((x, y, xx, yy), at_y) {
            intervals.push((from, to));
        }
    }
    beacons_at_y.sort();
    beacons_at_y.dedup();
    intervals.sort();

    let mut part1ans = 0;
    let mut next_uncovered = isize::MIN;
    for (from, to) in intervals {
        let start = std::cmp::max(from, next_uncovered);
        let end = std::cmp::max(to + 1, next_uncovered);
        while beacons_at_y.len() > 0 && beacons_at_y[0] < start {
            beacons_at_y.remove(0);
        }
        while beacons_at_y.len() > 0 && beacons_at_y[0] >= start && beacons_at_y[0] < end {
            part1ans -= 1;
            beacons_at_y.remove(0);
        }
        part1ans += end - start;
        next_uncovered = end;
    }

    println!("part1: {part1ans}");
}
