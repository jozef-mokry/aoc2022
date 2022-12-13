fn main() {
    let grid: Vec<Vec<u32>> = std::io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    assert!(grid.len() == grid[0].len());
    let mut up = grid.clone();
    let mut down = grid.clone();
    let mut left = grid.clone();
    let mut right = grid.clone();
    let size = grid.len() - 1;
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            up[i][j] = up[i][j].max(up[i - 1][j]);
            down[size - i][j] = down[size - i][j].max(down[size - i + 1][j]);
            left[i][j] = left[i][j].max(left[i][j - 1]);
            right[i][size - j] = right[i][size - j].max(right[i][size - j + 1]);
        }
    }
    let mut ans = 4 * grid.len() - 4;
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            if grid[i][j] > up[i - 1][j]
                || grid[i][j] > down[i + 1][j]
                || grid[i][j] > left[i][j - 1]
                || grid[i][j] > right[i][j + 1]
            {
                ans += 1;
            }
        }
    }
    println!("{ans}");

    // part 2:
    let mut scores = vec![vec![1; grid.len()]; grid.len()];

    // left
    for i in 0..grid.len() {
        let mut last_seen: [usize; 10] = [0; 10];
        for j in 0..grid[0].len() {
            let height = grid[i][j] as usize;
            let &end = last_seen[height..=9].iter().max().unwrap();
            scores[i][j] *= j - end;
            last_seen[height] = j;
            //if i == 0 || j == 0 || i == grid.len() - 1 || j == grid.len() - 1 {
            //    scores[i][j] = 0;
            //}
        }
    }
    println!("{scores:?}");
    // right
    for i in 0..grid.len() {
        let mut last_seen: [usize; 10] = [grid[0].len() - 1; 10];
        for j in (0..grid[0].len()).rev() {
            let height = grid[i][j] as usize;
            let &end = last_seen[height..=9].iter().min().unwrap();
            scores[i][j] *= end - j;
            last_seen[height] = j;
        }
    }
    println!("{scores:?}");
    // top
    for j in 0..grid[0].len() {
        let mut last_seen: [usize; 10] = [0; 10];
        for i in 0..grid.len() {
            let height = grid[i][j] as usize;
            let &end = last_seen[height..=9].iter().max().unwrap();
            scores[i][j] *= i - end;
            last_seen[height] = i;
        }
    }
    println!("{scores:?}");
    // bottom
    for j in 0..grid[0].len() {
        let mut last_seen: [usize; 10] = [grid.len() - 1; 10];
        for i in (0..grid.len()).rev() {
            let height = grid[i][j] as usize;
            let &end = last_seen[height..=9].iter().min().unwrap();
            scores[i][j] *= end - i;
            last_seen[height] = i;
        }
    }
    println!("{scores:?}");
    println!(
        "{}",
        scores
            .into_iter()
            .map(|row| row.into_iter().max().unwrap())
            .max()
            .unwrap()
    );
}
