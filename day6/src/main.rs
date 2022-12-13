use std::collections::{hash_map::Entry, HashMap};
use std::io;
fn main() {
    let line = io::stdin().lines().next().unwrap().unwrap();
    let mut seen: HashMap<u8, usize> = HashMap::new();
    let chars = line.as_bytes();
    let len = 14; // change to 4 for part 1
    for (i, &c) in chars.iter().enumerate() {
        *seen.entry(c).or_default() += 1;
        if i >= len {
            match seen.entry(chars[i - len]) {
                Entry::Occupied(e) if e.get() == &1 => {
                    e.remove();
                }
                Entry::Occupied(mut e) => *e.get_mut() -= 1,
                Entry::Vacant(_) => unreachable!(),
            }
        }
        if seen.len() == len {
            println!("{}", i + 1);
            return;
        }
    }
}
