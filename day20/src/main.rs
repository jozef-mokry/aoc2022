fn main() {
    let key = 811589153;
    let rounds = 10;
    let orig_nums: Vec<_> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<isize>().unwrap() * key)
        .enumerate()
        .collect();
    let mut nums = orig_nums.clone();

    let len_minus_one = nums.len() as isize - 1;
    for _ in 0..rounds {
        for &(idx, val) in orig_nums.iter() {
            let curr_idx = nums.iter().position(|&x| x == (idx, val)).unwrap();
            let shift = ((val % len_minus_one) + len_minus_one) % len_minus_one;
            let mut new_idx = (curr_idx as isize + shift) % nums.len() as isize;
            // println!("{curr_idx} shift: {shift}");
            if curr_idx as isize + shift >= nums.len() as isize {
                new_idx += 1; // for crossing the boundary
            }
            nums.remove(curr_idx);
            nums.insert(new_idx as usize, (idx, val));
            // println!("{:?}", nums.iter().map(|&(_, v)| v).collect::<Vec<_>>());
        }
    }
    let zero = nums.iter().position(|&(_, val)| val == 0).unwrap();
    let ans: isize = [
        nums[(zero + 1000) % nums.len()].1,
        nums[(zero + 2000) % nums.len()].1,
        nums[(zero + 3000) % nums.len()].1,
    ]
    .iter()
    .sum();
    println!("part1 {ans} ");
}
