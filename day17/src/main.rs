use std::collections::HashMap;
fn main() {
    let dirs = std::io::stdin().lines().next().unwrap().unwrap();
    let mut dirs = dirs.bytes().enumerate().cycle().peekable();

    let mut board = vec![
        b"xxxxxxxxx".to_owned(),
        b"x.......x".to_owned(),
        b"x.......x".to_owned(),
        b"x.......x".to_owned(),
    ];
    let blocks = [
        // ####
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        // .#.
        // ###
        // .#.
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        // ..#
        // ..#
        // ###
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        // #
        // #
        // #
        // #
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        // ##
        // ##
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    let mut highest = 0;
    let mut seen: HashMap<(u64, usize, usize), (usize, usize)> = HashMap::new();
    let mut round = 0;
    let mut blocks = blocks.iter().enumerate().cycle();
    // part 1:
    // let number_of_blocks = 2022;
    // part 2:
    let number_of_blocks = 1_000_000_000_000;
    let mut repeat_height = 0;
    let mut found_repeat = false;
    while round < number_of_blocks {
        let (block_idx, block) = blocks.next().unwrap();
        round += 1;
        println!("round: {round}");
        if !found_repeat {
            if let Some(reachable) = compute_reachable(&board, highest) {
                let &(dir_idx, _) = dirs.peek().unwrap();
                if let Some(&(prev_round, prev_height)) = seen.get(&(reachable, block_idx, dir_idx))
                {
                    found_repeat = true;
                    println!("found repetition: {prev_round}->{round}, {prev_height}->{highest}");
                    let blocks_per_repeat = round - prev_round;
                    let height_per_repeat = highest - prev_height;
                    let repeats = (number_of_blocks - round + 1) / blocks_per_repeat;
                    round += blocks_per_repeat * repeats;
                    repeat_height = height_per_repeat * repeats;
                    println!("round is now {round}");
                    if round > number_of_blocks {
                        break;
                    }
                }
                seen.insert((reachable, block_idx, dir_idx), (round, highest));
            }
        }
        // print(&board);
        // + 4 for starting 3 above highest point
        // + 4 for height of tallest block
        while board.len() < highest + 4 + 4 {
            board.push(b"x.......x".to_owned());
        }
        let mut pos = (highest + 4, 3);
        loop {
            let wind_pos = blow(pos, dirs.next().unwrap().1);
            if available(&board, &block, wind_pos) {
                pos = wind_pos;
            }
            let down_pos = (pos.0 - 1, pos.1);
            if available(&board, block, down_pos) {
                pos = down_pos;
            } else {
                highest = highest.max(draw(&mut board, block, pos));
                break;
            }
        }
    }

    println!("{}", highest + repeat_height);
}

fn blow((r, c): (usize, usize), wind_dir: u8) -> (usize, usize) {
    match wind_dir {
        b'>' => (r, c + 1),
        b'<' => (r, c - 1),
        _ => unreachable!(),
    }
}

fn available(board: &Vec<[u8; 9]>, block: &[(usize, usize)], (r, c): (usize, usize)) -> bool {
    block.iter().all(|(rr, cc)| board[r + rr][c + cc] == b'.')
}

fn draw(board: &mut Vec<[u8; 9]>, block: &[(usize, usize)], (r, c): (usize, usize)) -> usize {
    let mut top = 0;
    for (rr, cc) in block {
        board[r + rr][c + cc] = b'x';
        top = top.max(r + rr);
    }
    top
}

fn print(board: &Vec<[u8; 9]>) {
    for row in board.into_iter().rev() {
        println!("{:?}", std::str::from_utf8(row).unwrap());
    }
    println!("");
}

fn compute_reachable(board: &Vec<[u8; 9]>, row: usize) -> Option<u64> {
    let mut reachable = 0;
    let mut queue: Vec<(isize, isize)> = (1..=7).map(|x| (row as isize, x)).collect();
    while let Some((r, c)) = queue.pop() {
        if r < 0 || r >= board.len() as isize || c < 0 || c >= board[r as usize].len() as isize {
            continue;
        }
        let (r, c) = (r as usize, c as usize);
        if board[r][c] == b'x' {
            continue;
        }
        let idx = (row - r) * 7 + (c - 1);
        if idx >= 64 {
            return None;
        }
        if reachable & (1 << idx) != 0 {
            continue;
        }
        reachable |= 1 << idx;
        let (r, c) = (r as isize, c as isize);
        queue.extend([(r, c + 1), (r, c - 1), (r - 1, c)].into_iter())
    }
    return Some(reachable);
}
