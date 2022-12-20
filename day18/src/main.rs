use std::collections::HashSet;
fn main() {
    let (mut minn, mut maxx) = (isize::MAX, isize::MIN);
    let cubes: HashSet<(isize, isize, isize)> = std::io::stdin()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut xs = l.split(",");
            (
                xs.next().unwrap().parse::<isize>().unwrap(),
                xs.next().unwrap().parse::<isize>().unwrap(),
                xs.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .inspect(|&(x, y, z)| {
            minn = minn.min(x);
            minn = minn.min(y);
            minn = minn.min(z);
            maxx = maxx.max(x);
            maxx = maxx.max(y);
            maxx = maxx.max(z);
        })
        .collect();
    // part1:
    println!("part 1:{}", count_visible(&cubes));
    println!("{:?}", (minn, maxx));
    let (minn, maxx) = (minn - 1, maxx + 1);
    let mut visited = HashSet::new();
    fn dfs(
        (x, y, z): (isize, isize, isize),
        minn: isize,
        maxx: isize,
        visited: &mut HashSet<(isize, isize, isize)>,
        occupied: &HashSet<(isize, isize, isize)>,
    ) {
        if [x, y, z].iter().min().unwrap() < &minn
            || [x, y, z].iter().max().unwrap() > &maxx
            || visited.contains(&(x, y, z))
            || occupied.contains(&(x, y, z))
        {
            return;
        }
        visited.insert((x, y, z));
        for &(dx, dy, dz) in &[
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            dfs((x + dx, y + dy, z + dz), minn, maxx, visited, occupied);
        }
    }
    dfs((minn, minn, minn), minn, maxx, &mut visited, &cubes);
    let enclosing_cube_surface = 6 * (maxx - minn + 1) * (maxx - minn + 1);
    println!(
        "part2: {}",
        count_visible(&visited) - enclosing_cube_surface as usize
    );
}

fn count_visible(cubes: &HashSet<(isize, isize, isize)>) -> usize {
    let mut ans = 0;
    for &(x, y, z) in cubes {
        for &(dx, dy, dz) in &[
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            if !cubes.contains(&(x + dx, y + dy, z + dz)) {
                ans += 1;
            }
        }
    }
    return ans;
}
